mod control;

use control::C0;
use crosswords::Crosswords;
use std::io::{BufReader, Read};
use std::sync::Arc;
use std::sync::Mutex;
use teletypewriter::Process;
// https://vt100.net/emu/dec_ansi_parser
use vte::{Params, Parser};

pub type Square = crosswords::square::Square;
pub type Row = crosswords::row::Row<Square>;
pub type VisibleRows = Arc<Mutex<Vec<Row>>>;

pub trait Handler {
    /// A character to be displayed.
    fn input(&mut self, _c: char) {}
}

struct Performer {
    arc: VisibleRows,
    handler: Crosswords,
}

impl Performer {
    fn new(arc: VisibleRows, columns: usize, rows: usize) -> Performer {
        let crosswords: Crosswords = Crosswords::new(columns, rows);

        Performer {
            arc,
            handler: crosswords,
        }
    }
}

impl vte::Perform for Performer {
    fn print(&mut self, c: char) {
        // println!("[print] {c:?}");
        self.handler.input(c);

        let mut s = self.arc.lock().unwrap();
        // let visible_rows_to_string = self.handler.visible_rows_to_string();
        // let data_ptr: Arc<Mutex<Vec<&Square>>> = Arc::new(Mutex::from(self.handler.visible_rows()));

        *s = self.handler.visible_rows();

        // let s = &mut *self.arc.lock().unwrap();
        // s.push(c);
    }

    fn execute(&mut self, byte: u8) {
        println!("[execute] {byte:04x}");

        match byte {
            C0::HT => self.handler.put_tab(1),
            C0::BS => self.handler.backspace(),
            C0::CR => self.handler.carriage_return(),
            C0::LF | C0::VT | C0::FF => self.handler.linefeed(),
            C0::BEL => self.handler.bell(),
            C0::SUB => self.handler.substitute(),
            // C0::SI => self.handler.set_active_charset(CharsetIndex::G0),
            // C0::SO => self.handler.set_active_charset(CharsetIndex::G1),
            _ => println!("[unhandled] execute byte={byte:02x}"),
        }
    }

    fn hook(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
        println!(
            "[hook] params={params:?}, intermediates={intermediates:?}, ignore={ignore:?}, char={c:?}"
        );
    }

    fn put(&mut self, byte: u8) {
        println!("[put] {byte:02x}");
    }

    fn unhook(&mut self) {
        println!("[unhook]");
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        println!("[osc_dispatch] params={params:?} bell_terminated={bell_terminated}");
    }

    // Control Sequence Introducer
    // CSI is the two-character sequence ESCape left-bracket or the 8-bit
    // C1 code of 233 octal, 9B hex.  CSI introduces a Control Sequence, which
    // continues until an alphabetic character is received.
    fn csi_dispatch(
        &mut self,
        params: &Params,
        intermediates: &[u8],
        should_ignore: bool,
        action: char,
    ) {
        println!(
            "[csi_dispatch] params={params:#?}, intermediates={intermediates:?}, should_ignore={should_ignore:?}, action={action:?}"
        );

        if should_ignore || intermediates.len() > 1 {
            return;
        }

        let mut params_iter = params.iter();
        let handler = &mut self.handler;

        let mut next_param_or = |default: u16| match params_iter.next() {
            Some(&[param, ..]) if param != 0 => param,
            _ => default,
        };

        match (action, intermediates) {
            ('K', []) => handler.clear_line(next_param_or(0)),
            ('J', []) => {}
            _ => {}
        };
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        println!(
            "[esc_dispatch] intermediates={intermediates:?}, ignore={ignore:?}, byte={byte:02x}"
        );
    }
}

pub fn process(process: Process, data_ptr: VisibleRows, columns: usize, rows: usize) {
    let reader = BufReader::new(process);

    let mut handler = Performer::new(data_ptr, columns, rows);
    let mut parser = Parser::new();
    for byte in reader.bytes() {
        parser.advance(&mut handler, *byte.as_ref().unwrap());
    }
}
