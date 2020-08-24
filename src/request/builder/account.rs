use crate::response;
use crate::request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<response::Account> {}

impl<'t> VultrRequest<response::Account> for RequestBuilder<response::Account> {}
