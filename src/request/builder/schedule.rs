use crate::response;
use crate::request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<response::Schedule> {}

impl<'t> VultrRequest<response::Schedule> for RequestBuilder<response::Schedule> {}
