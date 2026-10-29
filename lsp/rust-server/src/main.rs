use rconfig::commands::Error;
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

    async fn did_open(&self, params: DidOpenTextDocumentParams) {}

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let document = params.text_document;

        let path = document.uri.to_file_path().expect("Expected a file URI").to_path_buf();

        match rconfig::commands::format(&path.to_str().expect("Should be a valid path")) {
            Ok(_) => {
                eprintln!("{path:?}");
            }
            Err(Error::File(e)) => {
                eprintln!("File error: {e}");
            }
            Err(Error::Grammar(e)) => {
                eprintln!("Syntax error: {e}");
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
