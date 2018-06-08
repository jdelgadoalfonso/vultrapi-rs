use request::{vultr, BaseRequest, VultrRequest};
use response::HeaderOnly;
use reqwest::Method;
use std::{fmt, marker::PhantomData};


pub struct RequestBuilder<'t, T> {
    pub api_key: &'t str,
    pub method: Method,
    pub url: String,
    pub resp_t: PhantomData<*const T>,
    pub body: Option<String>,
}

impl<'t, T> RequestBuilder<'t, T> {
    pub fn with_api_key(api_key: &'t str) -> RequestBuilder<'t, T> {
        RequestBuilder {
            api_key: api_key,
            method: Method::Get,
            url: String::new(),
            resp_t: PhantomData,
            body: None,
        }
    }

    pub fn new<S>(api_key: &'t str, url: S) -> RequestBuilder<'t, T>
        where S: Into<String>
    {
        RequestBuilder {
            api_key: api_key,
            method: Method::Get,
            url: url.into(),
            resp_t: PhantomData,
            body: None,
        }
    }
}

impl<'t, T> BaseRequest for RequestBuilder<'t, T> {
    fn api_key(&self) -> &str { self.api_key }
    fn url(&self) -> &str { &self.url[..] }
    fn method(&self) -> Method { self.method.clone() }
    fn body(&self) -> Option<String> { self.body.clone() }
}

impl<'t, T> fmt::Display for RequestBuilder<'t, T> {
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

impl<'t> VultrRequest<HeaderOnly> for RequestBuilder<'t, HeaderOnly> {
    fn retrieve(&self) -> Result<HeaderOnly, vultr::Error> {
        self.retrieve_header()
    }
}
