use response;
use request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<'t, response::Backup> {}

impl<'t> VultrRequest<response::Backup> for RequestBuilder<'t, response::Backup> {}

impl<'t> RequestBuilder<'t, response::Backups> {}

impl<'t> VultrRequest<response::Backups> for RequestBuilder<'t, response::Backups> {}
