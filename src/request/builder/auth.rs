use crate::response;
use crate::request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<response::Auth> {}

impl<'t> VultrRequest<response::Auth> for RequestBuilder<response::Auth> {}
