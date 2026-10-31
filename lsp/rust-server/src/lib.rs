pub mod backend;

use tower_lsp_server::{
    Client,
    ls_types::{ Diagnostic, DiagnosticSeverity, MessageType, Position, Range, Uri },
};

pub fn new_diagnostic(
    source: &str,
    span: std::ops::Range<usize>,
    message: String,
    severity: DiagnosticSeverity
) -> Diagnostic {
    let position_at = |offset: usize| {
        let offset = offset.min(source.len());

        let offset = (0..=offset)
            .rev()
            .find(|&i| source.is_char_boundary(i))
            .unwrap_or(0);

        let before = &source[..offset];

        let line = before.matches('\n').count();

        let line_start = before.rfind('\n').map_or(0, |i| i + 1);

        let character = source[line_start..offset].encode_utf16().count();

        Position {
            line: line as u32,
            character: character as u32,
        }
    };

    Diagnostic {
        range: Range {
            start: position_at(span.start),
            end: position_at(span.end),
        },
        severity: Some(severity),
        code: None,
        code_description: None,
        source: Some("rconfig".into()),
        message,
        related_information: None,
        tags: None,
        data: None,
    }
}

pub async fn to_file_path(uri: &Uri, client: &Client) -> Option<String> {
    let path = match uri.to_file_path() {
        Some(path) => path,
        None => {
            client.show_message(MessageType::ERROR, "rconfig: Expected a valid file URI").await;
            return None;
        }
    };

    let path_str = match path.to_str() {
        Some(path) => path,
        None => {
            client.show_message(MessageType::ERROR, "rconfig: Invalid file path encoding").await;
            return None;
        }
    };
    Some(path_str.to_string())
}
