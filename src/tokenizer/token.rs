use std::ops::Range;

#[derive(Debug)]
pub enum Token {
    Comment,
    Ident,
    String,
    Number,

    Equal,
    OpenSquare,
    CloseSquare,
    OpenBracket,
    CloseBracket,
    Comma,
    Minus,
    Dot,

    Newline,
    WhiteSpace,

    Unknown,
}

#[derive(Debug)]
pub struct SpannedToken {
    pub kind: Token,
    pub span: Range<usize>,
}
