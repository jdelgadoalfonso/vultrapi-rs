use crate::response;
use crate::request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<response::Region> {}

impl<'t> VultrRequest<response::Region> for RequestBuilder<response::Region> {}

impl<'t> RequestBuilder<response::Regions> {}

impl<'t> VultrRequest<response::Regions> for RequestBuilder<response::Regions> {}
