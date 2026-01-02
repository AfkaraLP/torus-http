//! This module handles parsing the client's request into a simple to work with data structure
use std::{collections::HashMap, str::FromStr};

use crate::method::HttpMethod;

#[derive(Clone, Debug, PartialEq, Eq)]
/// The incoming request
pub struct Request {
    /// i.e. Get, Post, etc...
    pub method: HttpMethod,
    /// path, currently including query parameters in the string
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub query: Option<HashMap<String, String>>,
}

impl FromStr for Request {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_line = s
            .lines()
            .next()
            .ok_or({
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "failed getting first http line",
                )
            })?
            .split_whitespace()
            .collect::<Vec<&str>>();
        let (method, path): (HttpMethod, String) = match first_line.as_slice() {
            [method_str, path, _version] => {
                (HttpMethod::from_str_val(method_str), (*path).to_string())
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "invalid http top header",
                ));
            }
        };
        let (path, query): (String, Option<HashMap<String, String>>) = match path.split_once('?') {
            Some((path, query)) => {
                let query_map: HashMap<String, String> = query
                    .split('&')
                    .filter_map(|q| q.split_once('='))
                    .map(|(k, v)| (k.to_owned(), v.to_owned()))
                    .collect();
                (path.to_owned(), Some(query_map))
            }
            None => (path, None),
        };

        let headers: HashMap<String, String> = s
            .lines()
            .take_while(|line| !line.is_empty())
            .skip(1)
            .filter_map(|line| {
                line.split_once(':')
                    .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
            })
            .collect();

        let body = if method == HttpMethod::Get {
            None
        } else {
            s.split_once("\r\n\r\n")
                .map(|v| v.1.to_owned())
                .filter(|v| !v.is_empty())
        };

        let req: Request = Request {
            method,
            path,
            headers,
            body,
            query,
        };
        Ok(req)
    }
}
