//! Unit and integration tests for ferrix-lsp (F6).

use ferrix_lsp::prelude::*;

#[test]
fn test_lsp_initialize_and_capabilities() {
    let mut server = LspServer::new();

    let init_req = JsonRpcMessage {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(1)),
        method: Some("initialize".to_string()),
        params: Some(serde_json::json!({})),
        result: None,
        error: None,
    };

    let resp = server.handle_message(init_req).expect("Response expected");
    assert_eq!(resp.id, Some(serde_json::json!(1)));
    let caps = &resp.result.unwrap()["capabilities"];
    assert_eq!(caps["hoverProvider"], true);
    assert_eq!(caps["documentFormattingProvider"], true);
}

#[test]
fn test_lsp_did_open_publishes_diagnostics() {
    let mut server = LspServer::new();

    let bad_code = r#"
export function Bad(): Element {
    if (x) {
        const s = useState(1);
    }
    return <Text>Bad</Text>;
}
"#;

    let did_open = JsonRpcMessage {
        jsonrpc: "2.0".to_string(),
        id: None,
        method: Some("textDocument/didOpen".to_string()),
        params: Some(serde_json::json!({
            "textDocument": {
                "uri": "file:///app/index.fsx",
                "text": bad_code
            }
        })),
        result: None,
        error: None,
    };

    let notif = server
        .handle_message(did_open)
        .expect("Diagnostic notification expected");
    assert_eq!(
        notif.method,
        Some("textDocument/publishDiagnostics".to_string())
    );
    let diags = notif.params.unwrap()["diagnostics"]
        .as_array()
        .unwrap()
        .clone();
    assert!(!diags.is_empty());
    assert_eq!(diags[0]["rule"], "rules-of-hooks");
}

#[test]
fn test_lsp_formatting_request() {
    let mut server = LspServer::new();
    let uri = "file:///app/test.fsx";

    let raw_code = r#"export function App(): Element {
return (
<Column>
<Text>Hi</Text>
</Column>
);
}
"#;

    // 1. Open document
    server.handle_message(JsonRpcMessage {
        jsonrpc: "2.0".to_string(),
        id: None,
        method: Some("textDocument/didOpen".to_string()),
        params: Some(serde_json::json!({
            "textDocument": {
                "uri": uri,
                "text": raw_code
            }
        })),
        result: None,
        error: None,
    });

    // 2. Request formatting
    let format_req = JsonRpcMessage {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(42)),
        method: Some("textDocument/formatting".to_string()),
        params: Some(serde_json::json!({
            "textDocument": { "uri": uri }
        })),
        result: None,
        error: None,
    };

    let resp = server
        .handle_message(format_req)
        .expect("Formatting response expected");
    let edits = resp.result.unwrap();
    let new_text = edits[0]["newText"].as_str().unwrap();
    assert!(new_text.contains("    return ("));
    assert!(new_text.contains("        <Column>"));
}
