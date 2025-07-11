# `neto`

[![Crates.io](https://img.shields.io/crates/v/neto.svg)](https://crates.io/crates/neto)
[![Docs.rs](https://docs.rs/neto/badge.svg)](https://docs.rs/neto)
[![License](https://img.shields.io/crates/l/neto.svg)](https://github.com/pas2rust/neto/blob/main/LICENSE)

**`neto`** is a Rust crate providing a flexible HTTP client abstraction with a focus on builder patterns, header management, and easy configuration of `reqwest::Client`. It supports both manual client injection and automatic client configuration with default headers.

---

## ✨ Features

- 🏗️ Fluent builder API for `Http` client with support for base URL, headers, and client.
- ⚙️ Automatic creation of `reqwest::Client` with default headers via `.config()`.
- 🔐 Easy injection of authorization tokens and other headers.
- 🔄 Support for both manual `Client` passing and automatic client configuration.
- 🚀 Async HTTP request helpers using `reqwest`.
- 🧪 Comprehensive testing with real API calls (e.g., PokeAPI).

---

## ⚙️ Installation

Add it to your `Cargo.toml`:

```bash
cargo add neto
```

## 🚀 Usage

### Manual Client injection

Create an Http instance by passing a manually created reqwest::Client. Headers are stored but not applied automatically to the client.

```rs
use neto::components::data::Http;
use reqwest::{Client, header::{HeaderValue, USER_AGENT}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let headers = vec![(USER_AGENT, HeaderValue::from_static("neto-http-test/1.0"))];

    let http = Http::new()
        .base_url("https://pokeapi.co/api/v2")
        .headers(headers)
        .client(Client::new())
        .build()
        .expect("Should build Http");

    let response = http.get("/pokemon/ditto", Vec::new()).await?;

    assert!(response.status().is_success());

    let json: serde_json::Value = response.json().await?;
    println!("Name: {}", json["name"]);

    Ok(())
}
```

### Automatic client creation with .config()

Build the Http struct without passing a client, then call .config() to create the reqwest::Client internally with all headers applied as default headers.

```rs
use neto::components::data::Http;
use reqwest::header::{HeaderValue, USER_AGENT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let headers = vec![(USER_AGENT, HeaderValue::from_static("neto-http-test/1.0"))];

    let mut http = Http::new()
        .base_url("https://pokeapi.co/api/v2")
        .headers(headers)
        .build()
        .expect("Should build Http");

    http.config().expect("Failed to configure Http");

    let response = http.get("/pokemon/ditto", Vec::new()).await?;

    assert!(response.status().is_success());

    let json: serde_json::Value = response.json().await?;
    println!("Name: {}", json["name"]);

    Ok(())
}
```

# ❤️ Donate

[![Monero](https://img.shields.io/badge/88NKLkhZf1nTVpaSU6vwG6dwBwb9tFVSM8Lpj3YqdL1PMt8Gm7opV7aUnMYBaAC9Y6a4kfDc3fLGoMVqeSJKNphyLpLdEvC-FF6600?style=flat&logo=monero&logoColor=white)](https://github.com/pas2rust/pas2rust/blob/main/pas-monero-donate.png)
[![Bitcoin](https://img.shields.io/badge/bc1qnlayyh84e9u5pd4m9g9sf4c5zdzswvkmudmdu5-EAB300?style=flat&logo=bitcoin&logoColor=white)](https://github.com/pas2rust/pas2rust/blob/main/pas-bitcoin-donate.png)