use response;

use request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<'t, response::Region> {}

impl<'t> VultrRequest<response::Region> for RequestBuilder<'t, response::Region> {}

impl<'t> RequestBuilder<'t, response::Regions> {}

impl<'t> VultrRequest<response::Regions> for RequestBuilder<'t, response::Regions> {}
