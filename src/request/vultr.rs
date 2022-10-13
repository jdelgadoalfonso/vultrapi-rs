use reqwest::{Client, Method, RequestBuilder, StatusCode};
use reqwest::header::{HeaderMap, CONTENT_TYPE, HeaderName};

use serde::de::DeserializeOwned;
use serde_json::{self, Value};

use crate::response::{HeaderOnly, NamedResponse};
use crate::ResultVultr;


pub trait BaseRequest {
    fn url(&self) -> &str;
    fn api_key(&self) -> &str;
    fn method(&self) -> Method;
    fn body(&self) -> Option<String>;
}

pub trait VultrRequest<T>: BaseRequest
where
    T: DeserializeOwned + NamedResponse + 'static
{
    fn request(&self) -> RequestBuilder {
        let client = Client::new();
        let req_builder = client.request(self.method(), self.url());
        let mut headers = HeaderMap::new();
        let api_key = HeaderName::from_static("api-key");

        headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
        headers.insert(api_key, self.api_key().parse().unwrap());

        if let Some(b) = self.body() {
            req_builder.body(b)
        } else {
            req_builder
        }.headers(headers)
    }

    fn retrieve_header(&self) -> ResultVultr<HeaderOnly> {
        let rb = self.request();
        let res = rb.send()?;
        let header = HeaderOnly::from_response(res)?;

        if header.raw_status != StatusCode::OK {
            Err(format_err!("{:?}", header.raw_status))
        } else {
            Ok(header)
        }
    }

    fn retrieve_json(&self) -> ResultVultr<String> {
        let rb = self.request();
        let content = rb.send()?
            .text()?;

        Ok(content)
    }

    fn retrieve(&self) -> ResultVultr<T> {
        let resp = self.retrieve_json()?;
        let v = serde_json::from_str::<Value>(resp.as_ref())?;
        let t = serde_json::from_value(v)?;

        Ok(t)
    }
}
