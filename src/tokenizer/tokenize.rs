#![allow(unused_parens)]
use super::{ SpannedToken, Token };

pub fn tokenize(source: &str) -> Vec<SpannedToken> {
    use super::Token::*;
    let mut tokens = Vec::new();
    let mut chars = source.char_indices().peekable();
    let mut line = 1;

    while let Some((start, ch)) = chars.next() {
        let token = match ch {
            '#' => tokenize_comment(start, &mut chars, line),
            '"' => tokenize_string(start, &mut chars, line),
            ch if ch.is_ascii_alphabetic() || ch == '_' =>
                tokenize_run(
                    start,
                    ch,
                    &mut chars,
                    |c| (c.is_ascii_alphanumeric() || c == '_'),
                    Ident,
                    line
                ),
            ch if ch.is_ascii_digit() => tokenize_number(start, ch, &mut chars, line, false),
            '\r' =>
                match chars.peek() {
                    Some(&(next_idx, '\n')) => {
                        chars.next();
                        line += 1;
                        SpannedToken {
                            kind: Newline,
                            span: start..next_idx + 1,
                            line: line - 1,
                        }
                    }
                    _ => {
                        line += 1;
                        SpannedToken {
                            kind: Newline,
                            span: start..start + 1,
                            line: line - 1,
                        }
                    }
                }
            ch if ch.is_whitespace() && ch != '\n' && ch != '\r' =>
                tokenize_run(
                    start,
                    ch,
                    &mut chars,
                    |c| c.is_whitespace() && c != '\n' && c != '\r',
                    WhiteSpace,
                    line
                ),
            '\n' => {
                line += 1;
                SpannedToken { kind: Newline, span: start..start + 1, line: line - 1 }
            }

            ',' => SpannedToken { kind: Comma, span: start..start + 1, line },
            '=' => SpannedToken { kind: Equal, span: start..start + 1, line },
            '[' => SpannedToken { kind: OpenSquare, span: start..start + 1, line },
            ']' => SpannedToken { kind: CloseSquare, span: start..start + 1, line },
            '{' => SpannedToken { kind: OpenBracket, span: start..start + 1, line },
            '}' => SpannedToken { kind: CloseBracket, span: start..start + 1, line },
            '-' =>
                match chars.peek() {
                    Some(&(_, next_ch)) if next_ch.is_ascii_digit() || next_ch == '.' =>
                        tokenize_number(start, ch, &mut chars, line, false),
                    _ => SpannedToken { kind: Minus, span: start..start + 1, line },
                }
            '.' => {
                match chars.peek() {
                    Some(&(_, next_ch)) if next_ch.is_ascii_digit() =>
                        tokenize_number(start, ch, &mut chars, line, true),
                    _ => SpannedToken { kind: Dot, span: start..start + 1, line },
                }
            }
            _ => SpannedToken { kind: Unknown, span: start..start + ch.len_utf8(), line },
        };
        tokens.push(token);
    }
    tokens.push(SpannedToken { kind: EOF, span: 0..0, line });

    tokens
}

fn tokenize_number<I>(
    start: usize,
    first: char,
    chars: &mut std::iter::Peekable<I>,
    line: usize,
    is_dot_before: bool
) -> SpannedToken
    where I: Iterator<Item = (usize, char)> + Clone
{
    use super::Token::*;
    let mut end = start + first.len_utf8();

    while let Some(&(next_idx, next_ch)) = chars.peek() {
        if !next_ch.is_ascii_digit() {
            break;
        }
        chars.next();
        end = next_idx + next_ch.len_utf8();
    }

    if is_dot_before {
        SpannedToken { kind: Float, span: start..end, line }
    } else if
        matches!(chars.peek(), Some(&(_, '.'))) &&
        ({
            let mut lookahead = chars.clone();
            lookahead.next();
            matches!(lookahead.peek(), Some(&(_, ch)) if ch.is_ascii_digit() || ch.is_whitespace())
        })
    {
        let (dot_idx, _) = chars.next().unwrap();
        end = dot_idx + 1;
        while let Some(&(next_idx, next_ch)) = chars.peek() {
            if !next_ch.is_ascii_digit() {
                break;
            }
            chars.next();
            end = next_idx + next_ch.len_utf8();
        }
        SpannedToken { kind: Float, span: start..end, line }
    } else {
        SpannedToken { kind: Number, span: start..end, line }
    }
}

fn tokenize_comment<I>(
    start: usize,
    chars: &mut std::iter::Peekable<I>,
    line: usize
) -> SpannedToken
    where I: Iterator<Item = (usize, char)>
{
    use super::Token::*;
    let mut end = start + 1;
    while let Some(&(next_idx, next_ch)) = chars.peek() {
        if next_ch == '\n' || next_ch == '\r' {
            break;
        }
        chars.next();
        end = next_idx + next_ch.len_utf8();
    }
    SpannedToken { kind: Comment, span: start..end, line }
}

fn tokenize_string<I>(start: usize, chars: &mut std::iter::Peekable<I>, line: usize) -> SpannedToken
    where I: Iterator<Item = (usize, char)>
{
    use super::Token::*;
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
    SpannedToken { kind: String, span: start + 1..end, line }
}

fn tokenize_run<I, F>(
    start: usize,
    ch: char,
    chars: &mut std::iter::Peekable<I>,
    mut check: F,
    kind: Token,
    line: usize
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
    SpannedToken { kind, span: start..end, line }
}
