use std::fmt;
use std::ops::Range;

#[derive(PartialEq, Clone, Copy)]
pub enum Token {
    Comment,
    Ident,
    String,
    Number,
    Float,

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

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Token::Comment => "comment",
            Token::Ident => "identifier",
            Token::String => "string",
            Token::Number => "number",
            Token::Float => "float",
            Token::Equal => "`=`",
            Token::OpenSquare => "`[`",
            Token::CloseSquare => "`]`",
            Token::OpenBracket => "`{`",
            Token::CloseBracket => "`}`",
            Token::Comma => "`,`",
            Token::Minus => "`-`",
            Token::Dot => "`.`",
            Token::Newline => "newline",
            Token::WhiteSpace => "whitespace",
            Token::Unknown => "unknown",
            Token::EOF => "end of file",
        };
        write!(f, "{name}")
    }
}

#[derive(Debug)]
pub struct SpannedToken {
    pub kind: Token,
    pub span: Range<usize>,
    pub line: usize,
}
