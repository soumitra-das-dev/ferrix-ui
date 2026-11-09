use ferrix_network::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Product {
    id: u64,
    name: String,
    price: f32,
}

#[test]
fn test_mock_http_client_get_json() {
    let client = MockHttpClient::new();
    let product = Product {
        id: 1,
        name: "Ferrix Pro Phone".to_string(),
        price: 999.0,
    };
    let json_bytes = serde_json::to_vec(&product).unwrap();

    client.register(
        "https://api.ferrix.dev/products/1",
        HttpResponse::new(200, json_bytes),
    );

    let req = HttpRequest::get("https://api.ferrix.dev/products/1");
    let resp = client.execute(req).expect("Request failed");
    assert_eq!(resp.status, 200);

    let parsed: Product = resp.json().expect("JSON parse failed");
    assert_eq!(parsed, product);
}

#[test]
fn test_mock_http_client_404_not_found() {
    let client = MockHttpClient::new();
    let req = HttpRequest::get("https://api.ferrix.dev/unknown");
    let err = client.execute(req).unwrap_err();
    assert!(matches!(err, NetworkError::RequestFailed(msg) if msg.contains("404")));
}
