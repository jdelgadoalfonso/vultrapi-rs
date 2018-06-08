use response;
use request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<'t, response::Application> {}

impl<'t> VultrRequest<response::Application> for RequestBuilder<'t, response::Application> {}

impl<'t> RequestBuilder<'t, response::Applications> {}

impl<'t> VultrRequest<response::Applications> for RequestBuilder<'t, response::Applications> {}
