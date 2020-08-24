use crate::response;
use crate::request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<response::Plan> {}

impl<'t> VultrRequest<response::Plan> for RequestBuilder<response::Plan> {}

impl<'t> RequestBuilder<response::Plans> {}

impl<'t> VultrRequest<response::Plans> for RequestBuilder<response::Plans> {}
