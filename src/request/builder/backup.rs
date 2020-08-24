use crate::response;
use crate::request::{VultrRequest, RequestBuilder};


impl<'t> RequestBuilder<response::Backup> {}

impl<'t> VultrRequest<response::Backup> for RequestBuilder<response::Backup> {}

impl<'t> RequestBuilder<response::Backups> {}

impl<'t> VultrRequest<response::Backups> for RequestBuilder<response::Backups> {}
