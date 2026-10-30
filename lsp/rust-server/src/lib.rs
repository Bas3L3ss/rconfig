use tower_lsp_server::ls_types::{ Diagnostic, DiagnosticSeverity, Position, Range };

pub fn push_diagnostic(
    diagnostics: &mut Vec<Diagnostic>,
    source: &str,
    span: std::ops::Range<usize>,
    message: String
) {
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
            line: line.saturating_sub(1) as u32,
            character: character as u32,
        }
    };

    diagnostics.push(Diagnostic {
        range: Range {
            start: position_at(span.start),
            end: position_at(span.end),
        },
        severity: Some(DiagnosticSeverity::ERROR),
        code: None,
        code_description: None,
        source: Some("rconfig".into()),
        message,
        related_information: None,
        tags: None,
        data: None,
    });
}
