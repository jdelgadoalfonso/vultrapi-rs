use response;
use request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<'t, response::OperatingSystem> {}

impl<'t> VultrRequest<response::OperatingSystem> for RequestBuilder<'t, response::OperatingSystem> {}

impl<'t> RequestBuilder<'t, response::OperatingSystems> {}

impl<'t> VultrRequest<response::OperatingSystems> for RequestBuilder<'t, response::OperatingSystems> {}
