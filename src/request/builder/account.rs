use response;
use request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<'t, response::Account> {}

impl<'t> VultrRequest<response::Account> for RequestBuilder<'t, response::Account> {}
