use request::RequestBuilder;

use response;


/// The main structure through which all calls are made. This holds a slice of the API KEY
#[derive(Clone)]
pub struct VultrMgr<'t> {
    api_key: &'t str,
}

impl<'t> VultrMgr<'t> {
    /// Creates a new instance of `VultrMgr` with a string slice of your API KEY
    pub fn with_api_key(api_key: &'t str) -> VultrMgr<'t> { VultrMgr { api_key: api_key } }

    /// Returns a request that can be used to view account information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use vultrapi::VultrMgr;
    /// # use vultrapi::request::VultrRequest;
    /// let vultrmgr = VultrMgr::with_api_key("asfasdfasdf");
    /// match vultrmgr.account()
    ///     .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn account(&self) -> RequestBuilder<'t, response::Account> {
        RequestBuilder::new(self.api_key, "https://api.vultr.com/v1/account/info")
    }

    /// Returns a request that can be used to retrieve a list of available
    /// applications. These refer to applications that can be launched when
    /// creating a Vultr VPS.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use vultrapi::VultrMgr;
    /// # use vultrapi::request::VultrRequest;
    /// let vultrmgr = VultrMgr::with_api_key("asfasdfasdf");
    /// match vultrmgr.applications()
    ///     .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn applications(&self) -> RequestBuilder<'t, response::Applications> {
        RequestBuilder::new(self.api_key, "https://api.vultr.com/v1/app/list")
    }

    /// Returns a request that can be used to retrieve information about the
    /// current API Key.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use vultrapi::VultrMgr;
    /// # use vultrapi::request::VultrRequest;
    /// let vultrmgr = VultrMgr::with_api_key("asfasdfasdf");
    /// match vultrmgr.auth()
    ///     .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn auth(&self) -> RequestBuilder<'t, response::Auth> {
        RequestBuilder::new(self.api_key, "https://api.vultr.com/v1/auth/info")
    }

    /// Returns a request that can be used to list all backups on the current
    /// account.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use vultrapi::VultrMgr;
    /// # use vultrapi::request::VultrRequest;
    /// let vultrmgr = VultrMgr::with_api_key("asfasdfasdf");
    /// match vultrmgr.backups()
    ///     .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn backups(&self) -> RequestBuilder<'t, response::Backups> {
        RequestBuilder::new(self.api_key, "https://api.vultr.com/v1/backup/list")
    }

    /// Returns a request that can be used to retrieve a list of available
    /// operating systems.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use vultrapi::VultrMgr;
    /// # use vultrapi::request::VultrRequest;
    /// let vultrmgr = VultrMgr::with_api_key("asfasdfasdf");
    /// match vultrmgr.operating_systems()
    ///     .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn operating_systems(&self) -> RequestBuilder<'t, response::OperatingSystems> {
        RequestBuilder::new(self.api_key, "https://api.vultr.com/v1/os/list")
    }

    /// Returns a request that can be used to list all snapshots on the current
    /// account.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use vultrapi::VultrMgr;
    /// # use vultrapi::request::VultrRequest;
    /// let vultrmgr = VultrMgr::with_api_key("asfasdfasdf");
    /// match vultrmgr.snapshots()
    ///     .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn snapshots(&self) -> RequestBuilder<'t, response::Snapshots> {
        RequestBuilder::new(self.api_key, "https://api.vultr.com/v1/snapshot/list")
    }

    /// Returns a request that can be used to list all active or pending
    /// virtual machines on the current account.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use vultrapi::VultrMgr;
    /// # use vultrapi::request::VultrRequest;
    /// let vultrmgr = VultrMgr::with_api_key("asfasdfasdf");
    /// match vultrmgr.servers()
    ///     .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn servers(&self) -> RequestBuilder<'t, response::Servers> {
        RequestBuilder::new(self.api_key, "https://api.vultr.com/v1/server/list")
    }

    /// Returns a request that can be used to retrieve a list of all active
    /// regions. Note that just because a region is listed here, does not mean
    /// that there is room for new servers.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use vultrapi::VultrMgr;
    /// # use vultrapi::request::VultrRequest;
    /// let vultrmgr = VultrMgr::with_api_key("asfasdfasdf");
    /// match vultrmgr.regions()
    ///     .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn regions(&self) -> RequestBuilder<'t, response::Regions> {
        RequestBuilder::new(self.api_key, "https://api.vultr.com/v1/regions/list")
    }

    /// Returns a request that can be used to retrieve a list of all active
    /// plans. Plans that are no longer available will not be shown.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use vultrapi::VultrMgr;
    /// # use vultrapi::request::VultrRequest;
    /// let vultrmgr = VultrMgr::with_api_key("asfasdfasdf");
    /// match vultrmgr.plans()
    ///     .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn plans(&self) -> RequestBuilder<'t, response::Plans> {
        RequestBuilder::new(self.api_key, "https://api.vultr.com/v1/plans/list")
    }
}
