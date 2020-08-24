use crate::response;
use crate::request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<response::OperatingSystem> {}

impl<'t> VultrRequest<response::OperatingSystem> for RequestBuilder<response::OperatingSystem> {}

impl<'t> RequestBuilder<response::OperatingSystems> {}

impl<'t> VultrRequest<response::OperatingSystems> for RequestBuilder<response::OperatingSystems> {}
