use response;
use request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<'t, response::Schedule> {}

impl<'t> VultrRequest<response::Schedule> for RequestBuilder<'t, response::Schedule> {}
