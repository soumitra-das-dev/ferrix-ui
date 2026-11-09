//! Language Server Protocol (LSP) request handler for Ferrix .fsx files.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use ferrix_fmt::format_source;
use ferrix_lint::engine::LintEngine;

/// JSON-RPC message envelope.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcMessage {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: Option<String>,
    pub params: Option<Value>,
    pub result: Option<Value>,
    pub error: Option<Value>,
}

pub struct LspServer {
    documents: HashMap<String, String>,
    linter: LintEngine,
}

impl Default for LspServer {
    fn default() -> Self {
        Self::new()
    }
}

impl LspServer {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            linter: LintEngine::new(),
        }
    }

    /// Handles an incoming JSON-RPC request and returns an optional response message.
    pub fn handle_message(&mut self, msg: JsonRpcMessage) -> Option<JsonRpcMessage> {
        let method = msg.method.as_deref()?;

        match method {
            "initialize" => {
                let result = serde_json::json!({
                    "capabilities": {
                        "textDocumentSync": 1, // Full sync
                        "hoverProvider": true,
                        "documentFormattingProvider": true
                    },
                    "serverInfo": {
                        "name": "ferrix-lsp",
                        "version": "0.1.0"
                    }
                });
                Some(JsonRpcMessage {
                    jsonrpc: "2.0".to_string(),
                    id: msg.id,
                    method: None,
                    params: None,
                    result: Some(result),
                    error: None,
                })
            }
            "textDocument/didOpen" => {
                if let Some(params) = msg.params {
                    if let Some(uri) = params["textDocument"]["uri"].as_str() {
                        if let Some(text) = params["textDocument"]["text"].as_str() {
                            self.documents.insert(uri.to_string(), text.to_string());
                            let diagnostics = self.linter.lint(text);
                            let notif = serde_json::json!({
                                "uri": uri,
                                "diagnostics": diagnostics
                            });
                            return Some(JsonRpcMessage {
                                jsonrpc: "2.0".to_string(),
                                id: None,
                                method: Some("textDocument/publishDiagnostics".to_string()),
                                params: Some(notif),
                                result: None,
                                error: None,
                            });
                        }
                    }
                }
                None
            }
            "textDocument/formatting" => {
                let uri = msg
                    .params
                    .as_ref()
                    .and_then(|p| p["textDocument"]["uri"].as_str())?;
                let doc = self.documents.get(uri)?;
                let formatted = format_source(doc);

                let edits = serde_json::json!([{
                    "range": {
                        "start": { "line": 0, "character": 0 },
                        "end": { "line": 99999, "character": 0 }
                    },
                    "newText": formatted
                }]);

                Some(JsonRpcMessage {
                    jsonrpc: "2.0".to_string(),
                    id: msg.id,
                    method: None,
                    params: None,
                    result: Some(edits),
                    error: None,
                })
            }
            "textDocument/hover" => {
                let hover_info = serde_json::json!({
                    "contents": {
                        "kind": "markdown",
                        "value": "### Ferrix UI Element\nAOT-compiled declarative element rendered natively on Android."
                    }
                });
                Some(JsonRpcMessage {
                    jsonrpc: "2.0".to_string(),
                    id: msg.id,
                    method: None,
                    params: None,
                    result: Some(hover_info),
                    error: None,
                })
            }
            _ => None,
        }
    }
}
