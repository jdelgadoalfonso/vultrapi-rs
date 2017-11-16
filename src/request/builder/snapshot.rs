use response;

use request::{VultrRequest, RequestBuilder};

use reqwest::Method;

use serde_urlencoded;

use std::marker::PhantomData;


impl<'t> RequestBuilder<'t, response::CreatedSnapshot> {}

impl<'t> VultrRequest<response::CreatedSnapshot> for RequestBuilder<'t, response::CreatedSnapshot> {}

impl<'t> RequestBuilder<'t, response::Snapshot> {}

impl<'t> VultrRequest<response::Snapshot> for RequestBuilder<'t, response::Snapshot> {}

impl<'t> RequestBuilder<'t, response::Snapshots> {
    pub fn create(self, sub_id: &str, desc: Option<&str>) ->
    RequestBuilder<'t, response::CreatedSnapshot>
    {
        // POST: "https://api.vultr.com/v1/snapshot/create"
        // body: "SUBID=1&description=blablabla"
        debug!("Create new Server");
        let params = &[("SUBID", Some(sub_id)), ("description", desc)];
        let body = serde_urlencoded::to_string(params).unwrap();
        RequestBuilder {
            method: Method::Post,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/snapshot/create"),
            resp_t: PhantomData,
            body: Some(body),
        }
    }

    pub fn destroy(self, snapshot_id: &str) ->
    RequestBuilder<'t, response::HeaderOnly>
    {
        // POST: "https://api.vultr.com/v1/snapshot/destroy"
        // body: "SNAPSHOTID=1"
        debug!("Create new Server");
        RequestBuilder {
            method: Method::Post,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/snapshot/destroy"),
            resp_t: PhantomData,
            body: Some(format!("SNAPSHOTID={}", snapshot_id)),
        }
    }
}

impl<'t> VultrRequest<response::Snapshots> for RequestBuilder<'t, response::Snapshots> {}
