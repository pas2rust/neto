use neto::components::data::Http;
use reqwest::{
    Client,
    header::{AUTHORIZATION, CONTENT_TYPE, HeaderValue, USER_AGENT},
};

fn dummy_client() -> Client {
    Client::new()
}

#[test]
fn http_builder_basic() {
    let headers = vec![
        (AUTHORIZATION, HeaderValue::from_static("Bearer token")),
        (CONTENT_TYPE, HeaderValue::from_static("application/json")),
    ];

    let http = Http::new()
        .base_url("https://example.com")
        .headers(headers.clone())
        .client(dummy_client())
        .build()
        .expect("Failed to build Http");

    assert_eq!(http.base_url, "https://example.com");
    assert_eq!(http.headers, headers);
}

#[test]
fn http_builder_missing_field_fails() {
    let result = Http::new().headers(vec![]).client(dummy_client()).build();

    assert!(result.is_err(), "Expected error when base_url is missing");
}

/*#[test]
fn http_builder_invalid_header() {
    let token = "token with spaces";
    let invalid_header = HeaderValue::from_str(token);
    assert!(
        invalid_header.is_err(),
        "Should fail due to invalid header value"
    );
}*/

/*#[test]
fn http_builder_with_empty_headers() {
    let http = Http::new()
        .base_url("http://localhost")
        .headers(vec![])
        .client(dummy_client())
        .build()
        .expect("Build should not fail with empty headers");

    assert_eq!(http.headers.len(), 0);
}*/

#[test]
fn http_builder_invalid_url_fails() {
    let headers = vec![(USER_AGENT, HeaderValue::from_static("test-agent"))];

    let result = Http::new()
        .base_url("not a url")
        .headers(headers)
        .client(dummy_client())
        .build();

    let err_msg = format!("{:?}", result.unwrap_err());
    assert!(
        err_msg.contains("Invalid URL format"),
        "Expected 'Invalid URL format' error, got: {}",
        err_msg
    );
}

#[tokio::test]
async fn http_real_get_pokeapi_ditto() {
    let headers = vec![(USER_AGENT, HeaderValue::from_static("neto-http-test/1.0"))];

    let http = Http::new()
        .base_url("https://pokeapi.co/api/v2")
        .headers(headers)
        .client(Client::new())
        .build()
        .expect("Should build Http");

    let response = http
        .get("/pokemon/ditto", Vec::new())
        .await
        .expect("GET request to PokeAPI failed");

    assert!(
        response.status().is_success(),
        "Expected 200 OK but got {}",
        response.status()
    );

    let json: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body as JSON");

    assert_eq!(json["name"], "ditto");
    assert!(
        json["abilities"].is_array(),
        "Expected 'abilities' to be an array"
    );
}

#[tokio::test]
async fn http_real_get_pokeapi_ditto_with_config() {
    use neto::components::data::Http;
    use reqwest::header::{HeaderValue, USER_AGENT};
    let headers = vec![(USER_AGENT, HeaderValue::from_static("neto-http-test/1.0"))];
    let mut http = Http::new()
        .base_url("https://pokeapi.co/api/v2")
        .headers(headers)
        .build()
        .expect("Should build Http");

    http.config()
        .expect("Failed to configure Http with headers");

    let response = http
        .get("/pokemon/ditto", Vec::new())
        .await
        .expect("GET request to PokeAPI failed");

    assert!(
        response.status().is_success(),
        "Expected 200 OK but got {}",
        response.status()
    );

    let json: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body as JSON");

    assert_eq!(json["name"], "ditto");
    assert!(
        json["abilities"].is_array(),
        "Expected 'abilities' to be an array"
    );
}
