use crate::components::data::{Body, Http};

impl Http {
    pub fn body(&self, req: reqwest::RequestBuilder, body: Body) -> reqwest::RequestBuilder {
        match body {
            #[cfg(feature = "json")]
            Body::JSON(json) => req.json(&json),
            #[cfg(not(feature = "json"))]
            Body::JSON(_) => panic!("JSON feature must be provided"),
            Body::BYTES(bytes) => req.body(bytes),
            Body::TXT(text) => req.body(text),
            Body::FORM(form) => req.form(&form),

            #[cfg(all(not(target_arch = "wasm32"), feature = "native"))]
            Body::MULTIPART(parts) => {
                let mut form = reqwest::multipart::Form::new();
                for (name, bytes, filename) in parts {
                    form = form.part(
                        name,
                        reqwest::multipart::Part::bytes(bytes).file_name(filename),
                    );
                }
                req.multipart(form)
            }

            #[cfg(any(target_arch = "wasm32", not(feature = "native")))]
            Body::MULTIPART(_) => {
                panic!("Multipart is not supported on this target. On WASM use web_sys::FormData.");
            }

            Body::None => req,
        }
    }
}
