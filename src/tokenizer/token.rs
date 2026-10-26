use std::ops::Range;

#[derive(Debug, PartialEq, Clone, Copy)]
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
    EOF,
}

#[derive(Debug)]
pub struct SpannedToken {
    pub kind: Token,
    pub span: Range<usize>,
    pub line: usize,
}
