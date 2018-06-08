use response;
use request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<'t, response::Auth> {}

impl<'t> VultrRequest<response::Auth> for RequestBuilder<'t, response::Auth> {}
