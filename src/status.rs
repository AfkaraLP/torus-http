//! Http status wrapper
use std::fmt::Display;

// TODO: yeah fill this out should not take long but too lazy rn
#[non_exhaustive]
#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub enum HttpStatus {
    Informational(InformationalResponse),
    Success(SuccessResponse),
    Redirection(RedirectionResponse),
    ClientError(ClientErrorResponse),
    ServerError(ServerErrorResponse),
}

impl Default for HttpStatus {
    fn default() -> Self {
        Self::Success(SuccessResponse::OK)
    }
}

#[non_exhaustive]
#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub enum InformationalResponse {
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,
}
#[non_exhaustive]
#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Debug, Default)]
pub enum SuccessResponse {
    #[default]
    OK = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,
}
#[non_exhaustive]
#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub enum RedirectionResponse {
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    SwitchProxy = 306,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
}
#[non_exhaustive]
#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub enum ClientErrorResponse {
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    URITooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableContent = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
}
#[non_exhaustive]
#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub enum ServerErrorResponse {
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpStatus::Informational(informational_response) => {
                write!(f, "{informational_response}")
            }
            HttpStatus::Success(success_response) => write!(f, "{success_response}"),
            HttpStatus::Redirection(redirection_response) => write!(f, "{redirection_response}"),
            HttpStatus::ClientError(client_error_response) => write!(f, "{client_error_response}"),
            HttpStatus::ServerError(server_error_response) => write!(f, "{server_error_response}"),
        }
    }
}

impl Display for ServerErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, num) = match self {
            ServerErrorResponse::InternalServerError => (
                "InternalServerError",
                ServerErrorResponse::InternalServerError as u16,
            ),
            ServerErrorResponse::NotImplemented => {
                ("NotImplemented", ServerErrorResponse::NotImplemented as u16)
            }
            ServerErrorResponse::BadGateway => {
                ("BadGateway", ServerErrorResponse::BadGateway as u16)
            }
            ServerErrorResponse::ServiceUnavailable => (
                "ServiceUnavailable",
                ServerErrorResponse::ServiceUnavailable as u16,
            ),
            ServerErrorResponse::GatewayTimeout => {
                ("GatewayTimeout", ServerErrorResponse::GatewayTimeout as u16)
            }
            ServerErrorResponse::HTTPVersionNotSupported => (
                "HTTPVersionNotSupported",
                ServerErrorResponse::HTTPVersionNotSupported as u16,
            ),
            ServerErrorResponse::VariantAlsoNegotiates => (
                "VariantAlsoNegotiates",
                ServerErrorResponse::VariantAlsoNegotiates as u16,
            ),
            ServerErrorResponse::InsufficientStorage => (
                "InsufficientStorage",
                ServerErrorResponse::InsufficientStorage as u16,
            ),
            ServerErrorResponse::LoopDetected => {
                ("LoopDetected", ServerErrorResponse::LoopDetected as u16)
            }
            ServerErrorResponse::NotExtended => {
                ("NotExtended", ServerErrorResponse::NotExtended as u16)
            }
            ServerErrorResponse::NetworkAuthenticationRequired => (
                "NetworkAuthenticationRequired",
                ServerErrorResponse::NetworkAuthenticationRequired as u16,
            ),
        };
        write!(f, "{num} {name}")
    }
}

impl Display for ClientErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, num) = match self {
            ClientErrorResponse::BadRequest => {
                ("BadRequest", ClientErrorResponse::BadRequest as u16)
            }
            ClientErrorResponse::Unauthorized => {
                ("Unauthorized", ClientErrorResponse::Unauthorized as u16)
            }
            ClientErrorResponse::PaymentRequired => (
                "PaymentRequired",
                ClientErrorResponse::PaymentRequired as u16,
            ),
            ClientErrorResponse::Forbidden => ("Forbidden", ClientErrorResponse::Forbidden as u16),
            ClientErrorResponse::NotFound => ("NotFound", ClientErrorResponse::NotFound as u16),
            ClientErrorResponse::MethodNotAllowed => (
                "MethodNotAllowed",
                ClientErrorResponse::MethodNotAllowed as u16,
            ),
            ClientErrorResponse::NotAcceptable => {
                ("NotAcceptable", ClientErrorResponse::NotAcceptable as u16)
            }
            ClientErrorResponse::ProxyAuthenticationRequired => (
                "ProxyAuthenticationRequired",
                ClientErrorResponse::ProxyAuthenticationRequired as u16,
            ),
            ClientErrorResponse::RequestTimeout => {
                ("RequestTimeout", ClientErrorResponse::RequestTimeout as u16)
            }
            ClientErrorResponse::Conflict => ("Conflict", ClientErrorResponse::Conflict as u16),
            ClientErrorResponse::Gone => ("Gone", ClientErrorResponse::Gone as u16),
            ClientErrorResponse::LengthRequired => {
                ("LengthRequired", ClientErrorResponse::LengthRequired as u16)
            }
            ClientErrorResponse::PreconditionFailed => (
                "PreconditionFailed",
                ClientErrorResponse::PreconditionFailed as u16,
            ),
            ClientErrorResponse::PayloadTooLarge => (
                "PayloadTooLarge",
                ClientErrorResponse::PayloadTooLarge as u16,
            ),
            ClientErrorResponse::URITooLong => {
                ("URITooLong", ClientErrorResponse::URITooLong as u16)
            }
            ClientErrorResponse::UnsupportedMediaType => (
                "UnsupportedMediaType",
                ClientErrorResponse::UnsupportedMediaType as u16,
            ),
            ClientErrorResponse::RangeNotSatisfiable => (
                "RangeNotSatisfiable",
                ClientErrorResponse::RangeNotSatisfiable as u16,
            ),
            ClientErrorResponse::ExpectationFailed => (
                "ExpectationFailed",
                ClientErrorResponse::ExpectationFailed as u16,
            ),
            ClientErrorResponse::ImATeapot => ("ImATeapot", ClientErrorResponse::ImATeapot as u16),
            ClientErrorResponse::MisdirectedRequest => (
                "MisdirectedRequest",
                ClientErrorResponse::MisdirectedRequest as u16,
            ),
            ClientErrorResponse::UnprocessableContent => (
                "UnprocessableContent",
                ClientErrorResponse::UnprocessableContent as u16,
            ),
            ClientErrorResponse::Locked => ("Locked", ClientErrorResponse::Locked as u16),
            ClientErrorResponse::FailedDependency => (
                "FailedDependency",
                ClientErrorResponse::FailedDependency as u16,
            ),
            ClientErrorResponse::TooEarly => ("TooEarly", ClientErrorResponse::TooEarly as u16),
            ClientErrorResponse::UpgradeRequired => (
                "UpgradeRequired",
                ClientErrorResponse::UpgradeRequired as u16,
            ),
            ClientErrorResponse::PreconditionRequired => (
                "PreconditionRequired",
                ClientErrorResponse::PreconditionRequired as u16,
            ),
            ClientErrorResponse::TooManyRequests => (
                "TooManyRequests",
                ClientErrorResponse::TooManyRequests as u16,
            ),
            ClientErrorResponse::RequestHeaderFieldsTooLarge => (
                "RequestHeaderFieldsTooLarge",
                ClientErrorResponse::RequestHeaderFieldsTooLarge as u16,
            ),
            ClientErrorResponse::UnavailableForLegalReasons => (
                "UnavailableForLegalReasons",
                ClientErrorResponse::UnavailableForLegalReasons as u16,
            ),
        };
        write!(f, "{num} {name}")
    }
}

impl Display for RedirectionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, num) = match self {
            RedirectionResponse::MultipleChoices => (
                "MultipleChoices",
                RedirectionResponse::MultipleChoices as u16,
            ),
            RedirectionResponse::MovedPermanently => (
                "MovedPermanently",
                RedirectionResponse::MovedPermanently as u16,
            ),
            RedirectionResponse::Found => ("Found", RedirectionResponse::Found as u16),
            RedirectionResponse::SeeOther => ("SeeOther", RedirectionResponse::SeeOther as u16),
            RedirectionResponse::NotModified => {
                ("NotModified", RedirectionResponse::NotModified as u16)
            }
            RedirectionResponse::UseProxy => ("UseProxy", RedirectionResponse::UseProxy as u16),
            RedirectionResponse::SwitchProxy => {
                ("SwitchProxy", RedirectionResponse::SwitchProxy as u16)
            }
            RedirectionResponse::TemporaryRedirect => (
                "TemporaryRedirect",
                RedirectionResponse::TemporaryRedirect as u16,
            ),
            RedirectionResponse::PermanentRedirect => (
                "PermanentRedirect",
                RedirectionResponse::PermanentRedirect as u16,
            ),
        };
        write!(f, "{num} {name}")
    }
}

impl Display for SuccessResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, num) = match self {
            SuccessResponse::OK => ("OK", SuccessResponse::OK as u16),
            SuccessResponse::Created => ("Created", SuccessResponse::Created as u16),
            SuccessResponse::Accepted => ("Accepted", SuccessResponse::Accepted as u16),
            SuccessResponse::NonAuthoritativeInformation => (
                "NonAuthoritativeInformation",
                SuccessResponse::NonAuthoritativeInformation as u16,
            ),
            SuccessResponse::NoContent => ("NoContent", SuccessResponse::NoContent as u16),
            SuccessResponse::ResetContent => ("ResetContent", SuccessResponse::ResetContent as u16),
            SuccessResponse::PartialContent => {
                ("PartialContent", SuccessResponse::PartialContent as u16)
            }
            SuccessResponse::MultiStatus => ("MultiStatus", SuccessResponse::MultiStatus as u16),
            SuccessResponse::AlreadyReported => {
                ("AlreadyReported", SuccessResponse::AlreadyReported as u16)
            }
            SuccessResponse::IMUsed => ("IMUsed", SuccessResponse::IMUsed as u16),
        };
        write!(f, "{num} {name}")
    }
}

impl Display for InformationalResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InformationalResponse::SwitchingProtocols => write!(
                f,
                "{} SwitchingProtocols",
                InformationalResponse::SwitchingProtocols
            ),
            InformationalResponse::Processing => {
                write!(f, "{} Processing", InformationalResponse::Processing)
            }
            InformationalResponse::EarlyHints => {
                write!(f, "{} EarlyHints", InformationalResponse::EarlyHints)
            }
        }
    }
}

impl From<ClientErrorResponse> for HttpStatus {
    fn from(val: ClientErrorResponse) -> Self {
        HttpStatus::ClientError(val)
    }
}
impl From<ServerErrorResponse> for HttpStatus {
    fn from(val: ServerErrorResponse) -> Self {
        HttpStatus::ServerError(val)
    }
}
impl From<InformationalResponse> for HttpStatus {
    fn from(val: InformationalResponse) -> Self {
        HttpStatus::Informational(val)
    }
}
impl From<RedirectionResponse> for HttpStatus {
    fn from(val: RedirectionResponse) -> Self {
        HttpStatus::Redirection(val)
    }
}
impl From<SuccessResponse> for HttpStatus {
    fn from(val: SuccessResponse) -> Self {
        HttpStatus::Success(val)
    }
}
