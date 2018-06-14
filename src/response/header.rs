use ::ResultVultr;
use hyper::header;
use response::{self, NamedResponse};
use reqwest::{Response, StatusCode};
use std::{borrow::Cow, fmt};


#[derive(Serialize, Deserialize)]
#[serde(remote = "StatusCode")]
enum StatusCodeDef {
    Continue,
    SwitchingProtocols,
    Processing,
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    ImUsed,
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    UriTooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
    Unregistered(u16),
}

#[derive(Serialize, Deserialize)]
pub struct HeaderOnly {
    #[serde(rename="content-type")]
    pub content_type: String,
    pub status: String,
    #[serde(with = "StatusCodeDef")]
    pub raw_status: StatusCode,
}

impl response::NotArray for HeaderOnly {}

impl HeaderOnly {
    pub fn from_response(r: Response) -> ResultVultr<HeaderOnly> {
        let c_type = match r.headers().get::<header::ContentType>() {
            Some(c) => c.to_string(),
            None => String::new(),
        };
        let raw_status = r.status();
        let status = format!("{}", raw_status);
        Ok(HeaderOnly {
            content_type: c_type,
            status: status,
            raw_status: raw_status,
        })
    }
}

impl NamedResponse for HeaderOnly {
    fn name<'a>() -> Cow<'a, str> { "header".into() }
}

impl fmt::Display for HeaderOnly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Status: {}", self.status)
    }
}

impl fmt::Debug for HeaderOnly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "content-type: {:?}\n\
                status: {:?}",
               self.content_type,
               self.status)
    }
}
