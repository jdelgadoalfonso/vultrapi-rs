use reqwest::Method;
use std::{fmt, marker::PhantomData};

use crate::ResultVultr;
use crate::request::{BaseRequest, VultrRequest};
use crate::response::HeaderOnly;

pub struct RequestBuilder<T> {
    pub api_key: String,
    pub method: Method,
    pub url: String,
    pub resp_t: PhantomData<T>,
    pub body: Option<String>,
}

impl<T> RequestBuilder<T> {
    pub fn with_api_key(api_key: &str) -> RequestBuilder<T> {
        RequestBuilder {
            api_key: String::from(api_key),
            method: Method::GET,
            url: String::new(),
            resp_t: PhantomData,
            body: None,
        }
    }

    pub fn new<S>(api_key: &str, url: S) -> RequestBuilder<T>
        where S: Into<String>
    {
        RequestBuilder {
            api_key: String::from(api_key),
            method: Method::GET,
            url: url.into(),
            resp_t: PhantomData,
            body: None,
        }
    }
}

impl<T> BaseRequest for RequestBuilder<T> {
    fn url(&self) -> &str { &self.url[..] }
    fn api_key(&self) -> &str { &self.api_key[..] }
    fn method(&self) -> Method { self.method.clone() }
    fn body(&self) -> Option<String> { self.body.clone() }
}

impl<T> fmt::Display for RequestBuilder<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "method: {}\n\
                content-type: application/json\n\
                API-Key: {}\n\
                url: {}\n\
                body: {}\n",
               self.method,
               self.api_key,
               if !self.url.is_empty() {
                   &self.url
               } else {
                   "None"
               },
               if let Some(ref bdy) = self.body {
                   bdy
               } else {
                   "None"
               })
    }
}

#[async_trait]
impl VultrRequest<HeaderOnly> for RequestBuilder<HeaderOnly> {
    async fn retrieve(&self) -> ResultVultr<HeaderOnly> {
        self.retrieve_header().await
    }
}
