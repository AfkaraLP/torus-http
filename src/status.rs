//! Http status wrapper
use std::fmt::Display;

// TODO: yeah fill this out should not take long but too lazy rn
#[non_exhaustive]
#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Debug, Default)]
pub enum HttpStatus {
    #[default]
    Ok = 200,
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpStatus::Ok => write!(f, "200 OK"),
        }
    }
}
