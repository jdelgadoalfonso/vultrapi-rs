use response;
use request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<'t, response::Plan> {}

impl<'t> VultrRequest<response::Plan> for RequestBuilder<'t, response::Plan> {}

impl<'t> RequestBuilder<'t, response::Plans> {}

impl<'t> VultrRequest<response::Plans> for RequestBuilder<'t, response::Plans> {}
