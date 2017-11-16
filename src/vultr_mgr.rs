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

    pub fn applications(&self) -> RequestBuilder<'t, response::Applications> {
        RequestBuilder::new(self.api_key, "https://api.vultr.com/v1/app/list")
    }

    pub fn auth(&self) -> RequestBuilder<'t, response::Auth> {
        RequestBuilder::new(self.api_key, "https://api.vultr.com/v1/auth/info")
    }

    pub fn backups(&self) -> RequestBuilder<'t, response::Backups> {
        RequestBuilder::new(self.api_key, "https://api.vultr.com/v1/backup/list")
    }
}
