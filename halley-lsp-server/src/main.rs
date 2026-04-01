use std::fs;
use std::path::Path;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(false)),
                    }),
                    file_operations: None,
                }),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                position_encoding: None,
                selection_range_provider: None,
                hover_provider: None,
                completion_provider: None,
                signature_help_provider: None,
                definition_provider: None,
                type_definition_provider: None,
                implementation_provider: None,
                references_provider: None,
                document_highlight_provider: None,
                document_symbol_provider: None,
                workspace_symbol_provider: None,
                code_action_provider: None,
                code_lens_provider: None,
                document_formatting_provider: None,
                document_range_formatting_provider: None,
                document_on_type_formatting_provider: None,
                rename_provider: None,
                document_link_provider: None,
                color_provider: None,
                folding_range_provider: None,
                declaration_provider: None,
                execute_command_provider: None,
                call_hierarchy_provider: None,
                semantic_tokens_provider: None,
                moniker_provider: None,
                linked_editing_range_provider: None,
                inline_value_provider: None,
                inlay_hint_provider: None,
                diagnostic_provider: None,
                experimental: None,
            },
            server_info: Some(ServerInfo {
                name: String::from("Halley Language Server"),
                version: Some(String::from("1.0.0")),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
    }
}

#[tokio::main]
async fn main() {
    let path = Path::new("lsp.sock");
    if path.exists() {
        fs::remove_file(path).unwrap();
    }

    let socket = tokio::net::UnixSocket::new_stream().unwrap();
    socket.bind(path).unwrap();

    let listener = socket.listen(1024).unwrap();

    loop {
        let (mut stream, _addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let (i, o) = stream.split();
            let (service, socket) = LspService::new(|client| Backend { client });
            Server::new(i, o, socket).serve(service).await;
        });
    }
}