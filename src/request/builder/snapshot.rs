use reqwest::Method;

use serde_urlencoded;
use std::marker::PhantomData;

use crate::request::{VultrRequest, RequestBuilder};
use crate::response;


impl<'t> RequestBuilder<response::CreatedSnapshot> {}

impl<'t> VultrRequest<response::CreatedSnapshot> for RequestBuilder<response::CreatedSnapshot> {}

impl<'t> RequestBuilder<response::Snapshot> {}

impl<'t> VultrRequest<response::Snapshot> for RequestBuilder<response::Snapshot> {}

impl<'t> RequestBuilder<response::Snapshots> {
    pub fn create(self, sub_id: &str, desc: Option<&str>) ->
    RequestBuilder<response::CreatedSnapshot>
    {
        // POST: "https://api.vultr.com/v1/snapshot/create"
        // body: "SUBID=1&description=blablabla"
        debug!("Create a new Snapshot");
        let params = &[("SUBID", Some(sub_id)), ("description", desc)];
        let body = serde_urlencoded::to_string(params).unwrap();
        RequestBuilder {
            method: Method::POST,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/snapshot/create"),
            resp_t: PhantomData,
            body: Some(body),
        }
    }

    pub fn destroy(self, snapshot_id: &str) ->
    RequestBuilder<response::HeaderOnly>
    {
        // POST: "https://api.vultr.com/v1/snapshot/destroy"
        // body: "SNAPSHOTID=1"
        debug!("Destroy a Snapshot");
        RequestBuilder {
            method: Method::POST,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/snapshot/destroy"),
            resp_t: PhantomData,
            body: Some(format!("SNAPSHOTID={}", snapshot_id)),
        }
    }
}

impl<'t> VultrRequest<response::Snapshots> for RequestBuilder<response::Snapshots> {}
