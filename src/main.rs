mod error;

use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tower_lsp::jsonrpc::Result;
use dashmap::DashMap;
use std::sync::Arc;

struct KymeraLanguageServer {
    client: Client,
    document_map: Arc<DashMap<String, String>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for KymeraLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![
                        ":".to_string(),  // For SPACS
                        ">".to_string(),  // For SPACS completion
                        "~".to_string(),  // For MUTA
                        "&".to_string(),  // For NMUT
                        "|".to_string(),  // For comments and AI features
                    ]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                // Add more capabilities as needed
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Kymera Language Server initialized!")
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let trigger_char = params
            .context
            .and_then(|ctx| ctx.trigger_character)
            .unwrap_or_default();

        let items = match trigger_char.as_str() {
            ":" => vec![
                CompletionItem::new_simple(":>".to_string(), "Scope resolution operator".to_string()),
            ],
            ">" => vec![
                CompletionItem::new_simple("des".to_string(), "Import declaration".to_string()),
                CompletionItem::new_simple("fnc".to_string(), "Function definition".to_string()),
                // Add more completions based on your KymeraConstruct enum
            ],
            "|" => vec![
                CompletionItem::new_simple("|>".to_string(), "Line comment".to_string()),
                CompletionItem::new_simple("|D>".to_string(), "Documentation comment".to_string()),
                CompletionItem::new_simple("|A>".to_string(), "AI-assisted code generation".to_string()),
            ],
            _ => vec![],
        };

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        // Implement hover based on KymeraConstruct documentation
        let hover_text = "Kymera language construct documentation";
        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: hover_text.to_string(),
            }),
            range: None,
        }))
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let content = params.content_changes[0].text.clone();
        self.document_map.insert(uri, content);
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text.clone();
        self.document_map.insert(uri, text);
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| KymeraLanguageServer {
        client,
        document_map: Arc::new(DashMap::new()),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
