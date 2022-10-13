use hyper::header;

use reqwest::{Response, StatusCode};
use reqwest::header::CONTENT_TYPE;

use std::{borrow::Cow, fmt};

use crate::ResultVultr;
use crate::response::{self, NamedResponse};


#[derive(Serialize, Deserialize)]
pub struct HeaderOnly {
    #[serde(rename="content-type")]
    pub content_type: String,
    pub status: String,
    #[serde(skip)]
    pub raw_status: StatusCode,
}

impl response::NotArray for HeaderOnly {}

impl HeaderOnly {
    pub fn from_response(r: Response) -> ResultVultr<HeaderOnly> {
        let c_type = match r.headers().get(CONTENT_TYPE) {
            Some(c) => String::from(c.to_str()?),
            None => String::new(),
        };
        let raw_status = r.status();
        let status = format!("{}", raw_status);
        Ok(HeaderOnly {
            content_type: c_type,
            status,
            raw_status,
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
