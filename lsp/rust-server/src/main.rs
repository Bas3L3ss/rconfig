use rconfig::{ commands::Error, file::open, grammar::GrammarError };
use rust_server::push_diagnostic;
use tower_lsp_server::{ jsonrpc::Result, ls_types::*, Client, LanguageServer, LspService, Server };

#[derive(Debug)]
struct Backend {
    client: Client,
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
        let path = match uri.to_file_path() {
            Some(path) => path,
            None => {
                self.client.show_message(
                    MessageType::ERROR,
                    "rconfig: Expected a valid file URI"
                ).await;
                return;
            }
        };

        let path_str = match path.to_str() {
            Some(path) => path,
            None => {
                self.client.show_message(
                    MessageType::ERROR,
                    "rconfig: Invalid file path encoding"
                ).await;
                return;
            }
        };

        self.client.publish_diagnostics(uri.clone(), Vec::new(), None).await;

        match rconfig::commands::validate(path_str) {
            Ok(_) => {}

            Err(Error::File(e)) => {
                self.client.show_message(
                    MessageType::ERROR,
                    format!("rconfig: File error: {e}")
                ).await;
            }

            Err(Error::Grammar(e)) => {
                let mut diagnostics = Vec::new();

                match &e {
                    | GrammarError::UnexpectedToken { found, .. }
                    | GrammarError::ExpectedExpression { found }
                    | GrammarError::InvalidNumber { token: found, .. } => {
                        push_diagnostic(
                            &mut diagnostics,
                            &params.text_document.text,
                            found.span.clone(),
                            e.to_string()
                        );
                    }

                    | GrammarError::DuplicatedKey { token, .. }
                    | GrammarError::MissingToken { token, .. }
                    | GrammarError::UnterminatedArray { token }
                    | GrammarError::UnterminatedStruct { token } => {
                        push_diagnostic(
                            &mut diagnostics,
                            &params.text_document.text,
                            token.span.clone(),
                            e.to_string()
                        );
                    }

                    GrammarError::UnexpectedEof { last } => {
                        push_diagnostic(
                            &mut diagnostics,
                            &params.text_document.text,
                            last.span.clone(),
                            e.to_string()
                        );
                    }
                }

                self.client.publish_diagnostics(uri, diagnostics, None).await;
            }
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;

        self.client.publish_diagnostics(uri.clone(), Vec::new(), None).await;

        let path = match uri.to_file_path() {
            Some(path) => path,
            None => {
                self.client.show_message(
                    MessageType::ERROR,
                    "rconfig: Expected a valid file URI"
                ).await;
                return;
            }
        };

        let path_str = match path.to_str() {
            Some(path) => path,
            None => {
                self.client.show_message(
                    MessageType::ERROR,
                    "rconfig: Invalid file path encoding"
                ).await;
                return;
            }
        };

        match rconfig::commands::format(path_str) {
            Ok(_) => {}

            Err(Error::File(e)) => {
                self.client.show_message(
                    MessageType::ERROR,
                    format!("rconfig: File error: {e}")
                ).await;
            }

            Err(Error::Grammar(e)) => {
                let source = match open(path_str) {
                    Ok(s) => s,
                    Err(e) => {
                        self.client.show_message(
                            MessageType::ERROR,
                            format!("rconfig: File error: {e}")
                        ).await;

                        return;
                    }
                };

                let mut diagnostics = Vec::new();

                match &e {
                    | GrammarError::UnexpectedToken { found, .. }
                    | GrammarError::ExpectedExpression { found }
                    | GrammarError::InvalidNumber { token: found, .. } => {
                        push_diagnostic(
                            &mut diagnostics,
                            &source,
                            found.span.clone(),
                            e.to_string()
                        );
                    }

                    | GrammarError::DuplicatedKey { token, .. }
                    | GrammarError::MissingToken { token, .. }
                    | GrammarError::UnterminatedArray { token }
                    | GrammarError::UnterminatedStruct { token } => {
                        push_diagnostic(
                            &mut diagnostics,
                            &source,
                            token.span.clone(),
                            e.to_string()
                        );
                    }

                    GrammarError::UnexpectedEof { last } => {
                        push_diagnostic(
                            &mut diagnostics,
                            &source,
                            last.span.clone(),
                            e.to_string()
                        );
                    }
                }

                self.client.publish_diagnostics(uri, diagnostics, None).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });

    Server::new(stdin, stdout, socket).serve(service).await;
}
