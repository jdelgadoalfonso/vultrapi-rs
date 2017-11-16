use hyper::header::{ContentType, Headers};

use response::{HeaderOnly, NamedResponse};

use reqwest::{self, Client, Method, RequestBuilder};

use serde::de::DeserializeOwned;

use serde_json::{self, Value};

use std::io::{self, Read};


error_chain!{
    foreign_links {
        Io(io::Error);
        Reqwest(reqwest::Error);
        SerdeJSON(serde_json::Error);
    }
}

header! { (APIKEY, "API-Key") => [String] }

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
        let mut req_builder = client.request(self.method(), self.url());
        let mut headers = Headers::new();

        if let Some(b) = self.body() {
            req_builder.body(b);
        }

        headers.set(ContentType("application/x-www-form-urlencoded".parse().unwrap()));
        headers.set(APIKEY(self.api_key().into()));

        req_builder.headers(headers);

        req_builder
    }

    fn retrieve_header(&self) -> Result<HeaderOnly> {
        let mut rb = self.request();
        let res = rb.send()?;
        let header = HeaderOnly::from_response(res)?;

        Ok(header)
    }

    fn retrieve_json(&self) -> Result<String> {
        let mut rb = self.request();
        let mut res = rb.send()?;
        let mut content = String::new();

        res.read_to_string(&mut content)?;

        Ok(content)
    }

    fn retrieve(&self) -> Result<T> {
        let resp = self.retrieve_json()?;
        let v = serde_json::from_str::<Value>(resp.as_ref())?;
        let t = serde_json::from_value(v)?;

        Ok(t)
    }
}
