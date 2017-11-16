use request::RequestBuilder;

use response;


#[derive(Clone)]
pub struct VultrMgr<'t> {
    api_key: &'t str,
}

impl<'t> VultrMgr<'t> {
    pub fn with_api_key(api_key: &'t str) -> VultrMgr<'t> { VultrMgr { api_key: api_key } }

    pub fn account(&self) -> RequestBuilder<'t, response::Account> {
        RequestBuilder::new(self.api_key, "https://api.vultr.com/v1/account/info")
    }
}
