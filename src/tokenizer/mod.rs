#![allow(unused_parens)]

pub mod token;
pub use token::*;

pub fn tokenize(source: &str) -> Vec<SpannedToken> {
    use Token::*;
    let mut tokens = Vec::new();
    let mut chars = source.char_indices().peekable();

    while let Some((start, ch)) = chars.next() {
        let token = match ch {
            '#' => tokenize_comment(start, &mut chars),
            '"' => tokenize_string(start, &mut chars),
            ch if ch.is_ascii_alphabetic() || ch == '_' =>
                tokenize_run(
                    start,
                    ch,
                    &mut chars,
                    |c| (c.is_ascii_alphanumeric() || c == '_'),
                    Ident
                ),
            ch if ch.is_ascii_digit() =>
                tokenize_run(start, ch, &mut chars, |c| c.is_ascii_digit(), Number),
            '\r' =>
                match chars.peek() {
                    Some(&(next_idx, '\n')) => {
                        chars.next();
                        SpannedToken {
                            kind: Newline,
                            span: start..next_idx + 1,
                        }
                    }
                    _ =>
                        SpannedToken {
                            kind: Newline,
                            span: start..start + 1,
                        },
                }
            ch if ch.is_whitespace() && ch != '\n' && ch != '\r' =>
                tokenize_run(
                    start,
                    ch,
                    &mut chars,
                    |c| c.is_whitespace() && c != '\n' && c != '\r',
                    WhiteSpace
                ),
            '\n' => SpannedToken { kind: Newline, span: start..start + 1 },

            ',' => SpannedToken { kind: Comma, span: start..start + 1 },
            '=' => SpannedToken { kind: Equal, span: start..start + 1 },
            '[' => SpannedToken { kind: OpenSquare, span: start..start + 1 },
            ']' => SpannedToken { kind: CloseSquare, span: start..start + 1 },
            '{' => SpannedToken { kind: OpenBracket, span: start..start + 1 },
            '}' => SpannedToken { kind: CloseBracket, span: start..start + 1 },
            '-' => SpannedToken { kind: Minus, span: start..start + 1 },
            '.' => SpannedToken { kind: Dot, span: start..start + 1 },
            _ => SpannedToken { kind: Unknown, span: start..start + ch.len_utf8() },
        };
        tokens.push(token);
    }
    tokens.push(SpannedToken { kind: EOF, span: 0..0 });

    tokens
}

fn tokenize_comment<I>(start: usize, chars: &mut std::iter::Peekable<I>) -> SpannedToken
    where I: Iterator<Item = (usize, char)>
{
    use Token::*;
    let mut end = start + 1;
    while let Some(&(next_idx, next_ch)) = chars.peek() {
        if next_ch == '\n' || next_ch == '\r' {
            break;
        }
        chars.next();
        end = next_idx + next_ch.len_utf8();
    }
    SpannedToken { kind: Comment, span: start..end }
}

fn tokenize_string<I>(start: usize, chars: &mut std::iter::Peekable<I>) -> SpannedToken
    where I: Iterator<Item = (usize, char)>
{
    use Token::*;
    let mut first_char = true;
    let mut end = start + 1;
    let mut escape = false;
    while let Some(&(next_idx, next_ch)) = chars.peek() {
        if first_char {
            first_char = false;
            continue;
        }
        if escape {
            chars.next();
            end = next_idx + next_ch.len_utf8();
            escape = false;
            continue;
        }
        if next_ch == '\\' {
            escape = true;
            end = next_idx + next_ch.len_utf8();
            chars.next();
            continue;
        }
        if next_ch == '"' {
            chars.next();
            break;
        }
        chars.next();
        end = next_idx + next_ch.len_utf8();
    }
    SpannedToken { kind: String, span: start + 1..end }
}

fn tokenize_run<I, F>(
    start: usize,
    ch: char,
    chars: &mut std::iter::Peekable<I>,
    mut check: F,
    kind: Token
)
    -> SpannedToken
    where I: Iterator<Item = (usize, char)>, F: FnMut(char) -> bool
{
    let mut end = start + ch.len_utf8();
    while let Some(&(next_idx, next_ch)) = chars.peek() {
        if !check(next_ch) {
            break;
        }
        chars.next();
        end = next_idx + next_ch.len_utf8();
    }
    SpannedToken { kind, span: start..end }
}
