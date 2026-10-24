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
            ch if ch.is_whitespace() && ch != '\n' =>
                tokenize_run(start, ch, &mut chars, |c| c.is_whitespace() && c != '\n', WhiteSpace),
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

    tokens
}

fn tokenize_comment<I>(start: usize, chars: &mut std::iter::Peekable<I>) -> SpannedToken
    where I: Iterator<Item = (usize, char)>
{
    use Token::*;
    let mut end = start + 1;
    while let Some(&(next_idx, next_ch)) = chars.peek() {
        if next_ch == '\n' {
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
    let mut end = start + 1;
    while let Some(&(next_idx, next_ch)) = chars.peek() {
        if next_ch == '"' {
            end = next_idx + 1;
            chars.next();
            break;
        }
        chars.next();
    }
    SpannedToken { kind: String, span: start..end }
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
