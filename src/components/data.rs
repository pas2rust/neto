use kenzu::Builder;
pub use reqwest::{
    Client,
    header::{HeaderName, HeaderValue},
};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Default, PartialEq)]
pub enum Body {
    #[default]
    None,
    JSON(Value),
    BYTES(Vec<u8>),
    TXT(String),
    FORM(HashMap<String, String>),
    MULTIPART(Vec<(String, Vec<u8>, String)>),
}

/// Structure representing HTTP configuration with base URL, headers, and client.
///
/// # Examples
///
/// ## 1. Using `.client()` manually
/// You create and pass a `reqwest::Client` manually. The headers inside the struct
/// are stored but not automatically applied to the client.
///
/// ```rust,no_run
/// use reqwest::{Client, header::{HeaderValue, USER_AGENT}};
/// use neto::components::data::Http;
///
/// fn main() {
///     let rt = tokio::runtime::Runtime::new().unwrap();
///     rt.block_on(async {
///         let headers = vec![
///             (USER_AGENT, HeaderValue::from_static("neto-http-test/1.0")),
///         ];
///
///         let http = Http::new()
///             .base_url("https://pokeapi.co/api/v2")
///             .headers(headers)
///             .client(Client::new()) // client created manually
///             .build()
///             .expect("Should build Http");
///
///         let response = http.get("/pokemon/ditto", Vec::new())
///             .await
///             .expect("GET request to PokeAPI failed");
///
///         assert!(response.status().is_success());
///     });
/// }
/// ```
///
/// ## 2. Using `.config()` to create the client internally
/// In this case, the `reqwest::Client` is not passed manually. After building the `Http`
/// struct, you call `.config()`, which creates the client internally and applies the
/// default headers stored in `Http.headers`.
///
/// ```rust,no_run
/// use reqwest::header::{HeaderValue, USER_AGENT};
/// use neto::components::data::Http;
///
/// fn main() {
///     let rt = tokio::runtime::Runtime::new().unwrap();
///     rt.block_on(async {
///         let headers = vec![
///             (USER_AGENT, HeaderValue::from_static("neto-http-test/1.0")),
///         ];
///
///         let mut http = Http::new()
///             .base_url("https://pokeapi.co/api/v2")
///             .headers(headers)
///             .build()
///             .expect("Should build Http");
///
///         http.config().expect("Failed to configure Http");
///
///         let response = http.get("/pokemon/ditto", Vec::new())
///             .await
///             .expect("GET request to PokeAPI failed");
///
///         assert!(response.status().is_success());
///     });
/// }
/// ```
///
/// # Key difference
///
/// - In the first example, the `reqwest::Client` is created externally and passed
/// manually to `Http`. The headers in the struct are not automatically applied to
/// the client.
///
/// - In the second example, the client is created internally by `Http::config()`,
/// which uses the stored headers to set default headers in the client.
///
/// Use `.config()` when you want the client to always include the stored headers,
/// such as authentication tokens, user-agent, etc., without creating the client
/// externally.
///
/// For simpler cases where the client is created separately, passing it manually
/// with `.client(Client::new())` may be sufficient.
#[derive(Debug, Builder, Clone)]
pub struct Http {
    #[build(
        pattern = r"^(https?://)?([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}(/.*)?$",
        err = "Invalid URL format"
    )]
    pub base_url: String,
    pub headers: Header,
    pub client: Client,
}

pub type Header = Vec<(HeaderName, HeaderValue)>;
pub type Query = Vec<(&'static str, &'static str)>;
