use colors::NamedColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Named(NamedColor),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Attr {
    /// Clear all special abilities.
    Reset,
    /// Bold text.
    Bold,
    /// Dim or secondary color.
    Dim,
    /// Italic text.
    Italic,
    /// Underline text.
    Underline,
    /// Underlined twice.
    DoubleUnderline,
    /// Undercurled text.
    Undercurl,
    /// Dotted underlined text.
    DottedUnderline,
    /// Dashed underlined text.
    DashedUnderline,
    /// Blink cursor slowly.
    BlinkSlow,
    /// Blink cursor fast.
    BlinkFast,
    /// Invert colors.
    Reverse,
    /// Do not display characters.
    Hidden,
    /// Strikeout text.
    Strike,
    /// Cancel bold.
    CancelBold,
    /// Cancel bold and dim.
    CancelBoldDim,
    /// Cancel italic.
    CancelItalic,
    /// Cancel all underlines.
    CancelUnderline,
    /// Cancel blink.
    CancelBlink,
    /// Cancel inversion.
    CancelReverse,
    /// Cancel text hiding.
    CancelHidden,
    /// Cancel strikeout.
    CancelStrike,
    /// Set indexed foreground color.
    Foreground(Color),
    /// Set indexed background color.
    Background(Color),
    /// Underline color.
    UnderlineColor(Option<Color>),
}
