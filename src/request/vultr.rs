use ::ResultVultr;
use hyper::{
    StatusCode, header::{CONTENT_TYPE, HeaderMap, HeaderName}
};
use response::{HeaderOnly, NamedResponse};
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde_json::{self, Value};
use std::io::Read;


pub trait BaseRequest {
    fn url(&self) -> &str;
    fn api_key(&self) -> &str;
    fn method(&self) -> Method;
    fn body(&self) -> Option<String>;
}

pub trait VultrRequest<T>: BaseRequest
    where T: DeserializeOwned + NamedResponse
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
        let mut res = rb.send()?;
        let mut content = String::new();

        res.read_to_string(&mut content)?;

        if res.status() != StatusCode::OK {
            Err(format_err!("{}", content))
        } else {
            Ok(content)
        }
    }

    fn retrieve(&self) -> ResultVultr<T> {
        let resp = self.retrieve_json()?;
        let v = serde_json::from_str::<Value>(resp.as_ref())?;
        let t = serde_json::from_value(v)?;

        Ok(t)
    }
}
