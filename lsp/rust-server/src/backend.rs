use crate::{ new_diagnostic, to_file_path };
use rconfig::{ commands::Error, file::open, grammar::GrammarError };
use tower_lsp_server::{ jsonrpc::Result, ls_types::*, Client, LanguageServer };

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
}

impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(
                    TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL)
                ),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "RCFG language server initialized").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let source = params.text_document.text;

        self.clear_diagnostics(&uri).await;

        let Some(path) = self.resolve_path(&uri).await else {
            return;
        };

        if let Err(error) = rconfig::commands::validate(&path) {
            match error {
                Error::File(e) => self.show_file_error(e).await,
                Error::Grammar(e) => {
                    self.publish_grammar_error(uri, &source, e).await;
                }
            }
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;

        self.clear_diagnostics(&uri).await;

        let Some(path) = self.resolve_path(&uri).await else {
            return;
        };

        if let Err(error) = rconfig::commands::format(&path) {
            match error {
                Error::File(e) => self.show_file_error(e).await,
                Error::Grammar(e) => {
                    let source = match open(&path) {
                        Ok(source) => source,
                        Err(e) => {
                            self.show_file_error(e).await;
                            return;
                        }
                    };

                    self.publish_grammar_error(uri.clone(), &source, e).await;
                }
            }
        }
        if let Err(error) = rconfig::commands::validate(&path) {
            match error {
                Error::File(e) => self.show_file_error(e).await,
                Error::Grammar(e) => {
                    let source = match open(&path) {
                        Ok(source) => source,
                        Err(e) => {
                            self.show_file_error(e).await;
                            return;
                        }
                    };

                    self.publish_grammar_error(uri, &source, e).await;
                }
            }
        }
    }
}

impl Backend {
    async fn clear_diagnostics(&self, uri: &Uri) {
        self.client.publish_diagnostics(uri.clone(), Vec::new(), None).await;
    }

    async fn show_file_error(&self, error: impl std::fmt::Display) {
        self.client.show_message(MessageType::ERROR, format!("rconfig: File error: {error}")).await;
    }

    async fn publish_grammar_error(&self, uri: Uri, source: &str, error: GrammarError) {
        let diagnostics = match &error {
            | GrammarError::UnexpectedToken { found, .. }
            | GrammarError::ExpectedExpression { found }
            | GrammarError::InvalidNumber { token: found, .. }
            | GrammarError::MissingToken { token: found, .. }
            | GrammarError::UnterminatedArray { token: found }
            | GrammarError::UnterminatedStruct { token: found }
            | GrammarError::UnexpectedEof { last: found } => {
                vec![
                    new_diagnostic(
                        source,
                        found.span.clone(),
                        error.to_string(),
                        DiagnosticSeverity::ERROR
                    )
                ]
            }

            GrammarError::DuplicatedKey { token: found, first, key } =>
                vec![
                    new_diagnostic(
                        source,
                        found.span.clone(),
                        error.to_string(),
                        DiagnosticSeverity::ERROR
                    ),
                    new_diagnostic(
                        source,
                        first.span.clone(),
                        format!("Key: \"{key}\" is already defined here"),
                        DiagnosticSeverity::HINT
                    )
                ],
        };

        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }

    async fn resolve_path(&self, uri: &Uri) -> Option<String> {
        to_file_path(uri, &self.client).await
    }
}
