//! This module handles parsing the client's request into a simple to work with data structure
use std::{collections::HashMap, str::FromStr};

use crate::method::HttpMethod;

#[derive(Clone, Debug, PartialEq, Eq)]
/// The incoming request
pub struct Request {
    /// i.e. Get, Post, etc...
    pub method: HttpMethod,
    /// Hath, currently including query parameters in the string
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    /// Parameter map after '?' in a request
    pub query: Option<HashMap<String, String>>,
}

impl FromStr for Request {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut first_line = input
            .lines()
            .next()
            .ok_or({
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "failed getting first http line",
                )
            })?
            .split_whitespace();

        let (Some(method_str), Some(path), Some(_version)) =
            (first_line.next(), first_line.next(), first_line.next())
        else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid http top header",
            ));
        };

        let method = HttpMethod::from_str_val(method_str);

        let (path, query): (&str, Option<HashMap<String, String>>) = match path.split_once('?') {
            Some((path, query)) => {
                let query_map: HashMap<String, String> = query
                    .split('&')
                    .filter_map(|q| q.split_once('='))
                    .map(|(k, v)| (k.to_owned(), v.to_owned()))
                    .collect();
                (path, Some(query_map))
            }
            None => (path, None),
        };

        let headers: HashMap<String, String> = input
            .lines()
            .take_while(|line| !line.is_empty())
            .skip(1)
            .filter_map(|line| {
                line.split_once(':')
                    .map(|(k, v)| (k.trim().to_owned(), v.trim().to_owned()))
            })
            .collect();

        let body = if method == HttpMethod::Get {
            None
        } else {
            input
                .split_once("\r\n\r\n")
                .map(|head_body_split| head_body_split.1.to_owned())
                .filter(|body| !body.is_empty())
        };

        let path = path.to_owned();

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
