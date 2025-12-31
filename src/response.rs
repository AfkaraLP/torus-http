//! Exports a trait for generating HTTP responses, with a simple default
//! implementation for string bodies.
//!
//! # Example:
//!
//! ```rust
//! let response = HttpResponse::new().set_body("hey there").insert_header("Cool-Header", "so cool");
//! let response = "hello".into_response();
//! ```

use std::collections::HashMap;

use crate::status::HttpStatus;

/// Trait that allows things to be sent back from the server
pub trait Response {
    fn to_response(&self) -> HttpResponse;
}

impl<T: AsRef<str>> Response for T {
    fn to_response(&self) -> HttpResponse {
        HttpResponse::new().set_body(self.as_ref())
    }
}

/// Struct that contains all the information that will be sent to the client
#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct HttpResponse {
    pub headers: HashMap<String, String>,
    pub status: HttpStatus,
    pub body: Option<String>,
}

impl HttpResponse {
    #[must_use]
    pub fn new_body(body: String, status: HttpStatus) -> Self {
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Content-Length".into(), body.chars().count().to_string());
        Self {
            headers,
            status,
            body: Some(body),
        }
    }

    #[must_use]
    pub fn insert_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    #[must_use]
    pub fn set_body(mut self, body: impl Into<String>) -> Self {
        let body = body.into();
        let body_len = body.chars().count();
        self.body.replace(body);
        self.headers
            .insert("Content-Length".into(), body_len.to_string());
        self
    }

    #[must_use]
    pub fn set_status(mut self, status: HttpStatus) -> Self {
        self.status = status;
        self
    }

    #[must_use]
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
            status: HttpStatus::Ok,
            body: None,
        }
    }

    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.into_string().into_bytes()
    }

    pub(crate) fn into_string(self) -> String {
        use std::fmt::Write;

        let headers = self.headers.iter().fold(String::new(), |mut acc, (k, v)| {
            acc.write_fmt(core::format_args!("{k}: {v}\r\n"))
                .expect("if this fails fuck clippy");
            acc
        });

        format!(
            "HTTP/1.1 {}\r\n{headers}\r\n{}",
            self.status,
            self.body.unwrap_or_default()
        )
    }
}
