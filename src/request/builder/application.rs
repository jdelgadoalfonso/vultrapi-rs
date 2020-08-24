use crate::response;
use crate::request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<response::Application> {}

impl<'t> VultrRequest<response::Application> for RequestBuilder<response::Application> {}

impl<'t> RequestBuilder<response::Applications> {}

impl<'t> VultrRequest<response::Applications> for RequestBuilder<response::Applications> {}
