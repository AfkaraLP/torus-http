//! Contains safe wrappers for http methods, note that the conversion is case insensitive so custom methods might not behave as expected
//!
//! # example:
//!
//! ```rust
//! assert_eq!(HttpMethod::Get, HttpMethod::from_str("GET"));
//! ```
//!
//! ## Note!!!
//!
//! due to how it is written right now, you cannot get a custom overwrite for the method (TODO: rephrase this I apparently don't know english)
//!
//! ```rust
//! assert_eq!(HttpMethod::Other("GET"), HttpMethod::from_str("GET")); // this does not hold
//! ```

impl HttpMethod {
    /// Generate an http method from a string
    #[must_use]
    pub fn from_str_val(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "get" => HttpMethod::Get,
            "post" => HttpMethod::Post,
            "update" => HttpMethod::Update,
            "delete" => HttpMethod::Delete,
            "put" => HttpMethod::Put,
            "patch" => HttpMethod::Patch,
            "head" => HttpMethod::Head,
            "options" => HttpMethod::Options,
            other => HttpMethod::Other(other.to_string()),
        }
    }
}

/// Enum covering most standard http methods and also allowing for custom ones
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum HttpMethod {
    Get,
    Post,
    Delete,
    Update,
    Put,
    Patch,
    Head,
    Options,
    Other(String),
}
