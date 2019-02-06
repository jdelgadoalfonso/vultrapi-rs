use response;
use request::{VultrRequest, RequestBuilder};
use reqwest::Method;
use serde_urlencoded;
use serde::ser::Serializer;
use std::marker::PhantomData;


#[derive(Serialize)]
pub struct ServerOptions<'t> {
    #[serde(rename="DCID")]
    pub dc_id: &'t str,
    #[serde(rename="VPSPLANID")]
    pub vps_plan_id: &'t str,
    #[serde(rename="OSID")]
    pub os_id: &'t str,
    #[serde(rename="SNAPSHOTID")]
    pub snapshot_id: Option<&'t str>,
    pub hostname: Option<&'t str>,
    pub label: Option<&'t str>,
    pub tag: Option<&'t str>,
    #[serde(serialize_with = "serialize_bool")]
    pub enable_private_network: Option<bool>,
}

#[derive(Serialize)]
pub struct ScheduleOptions<'t> {
    #[serde(rename="SUBID")]
    pub sub_id: &'t str,
    pub cron_type: &'t str, // TODO: Change to Enumeration type
    pub hour: Option<u32>,
    pub dow: Option<u32>,
    pub dom: Option<u32>,
}

fn serialize_bool<S>(x: &Option<bool>, s: S) -> Result<S::Ok, S::Error>
where S: Serializer,
{
    if let Some(v) = *x {
        let value = if v { "yes" } else { "no" };
        s.serialize_str(value)
    } else {
        s.serialize_none()
    }
}

impl<'t> RequestBuilder<'t, response::CreatedServer> {}

impl<'t> VultrRequest<response::CreatedServer> for RequestBuilder<'t, response::CreatedServer> {}

impl<'t> RequestBuilder<'t, response::Server> {}

impl<'t> VultrRequest<response::Server> for RequestBuilder<'t, response::Server> {}

impl<'t> RequestBuilder<'t, response::Servers> {
    pub fn create(self, server_opt: &ServerOptions) ->
    RequestBuilder<'t, response::CreatedServer>
    {
        // POST: "https://api.vultr.com/v1/server/create"
        // body: "DCID=1&VPSPLANID=1&OSID=164&SNAPSHOTID="
        debug!("Create new Server");
        RequestBuilder {
            method: Method::POST,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/server/create"),
            resp_t: PhantomData,
            body: Some(serde_urlencoded::to_string(server_opt).unwrap()),
        }
    }

    pub fn destroy(self, sub_id: &str) -> RequestBuilder<'t, response::HeaderOnly>  {
        // POST: "https://api.vultr.com/v1/server/destroy"
        // body: "SUBID=576965"
        debug!("Destroy Server: {}", sub_id);
        RequestBuilder {
            method: Method::POST,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/server/destroy"),
            resp_t: PhantomData,
            body: Some(format!("SUBID={}", sub_id)),
        }
    }

    pub fn reboot(self, sub_id: &str) -> RequestBuilder<'t, response::HeaderOnly>  {
        // POST: "https://api.vultr.com/v1/server/reboot"
        // body: "SUBID=576965"
        debug!("Reboot Server: {}", sub_id);
        RequestBuilder {
            method: Method::POST,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/server/reboot"),
            resp_t: PhantomData,
            body: Some(format!("SUBID={}", sub_id)),
        }
    }

    pub fn halt(self, sub_id: &str) -> RequestBuilder<'t, response::HeaderOnly> {
        // POST: "https://api.vultr.com/v1/server/halt"
        // body: "SUBID=576965"
        debug!("Halt Server: {}", sub_id);
        RequestBuilder {
            method: Method::POST,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/server/halt"),
            resp_t: PhantomData,
            body: Some(format!("SUBID={}", sub_id)),
        }
    }

    pub fn start(self, sub_id: &str) -> RequestBuilder<'t, response::HeaderOnly> {
        // POST: "https://api.vultr.com/v1/server/start"
        // body: "SUBID=576965"
        debug!("Start Server: {}", sub_id);
        RequestBuilder {
            method: Method::POST,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/server/start"),
            resp_t: PhantomData,
            body: Some(format!("SUBID={}", sub_id)),
        }
    }

    pub fn upgrade_plan(self, sub_id: &str, plan_id: &str) -> RequestBuilder<'t, response::HeaderOnly> {
        // POST: "https://api.vultr.com/v1/server/upgrade_plan"
        // body: "SUBID=576965&VPSPLANID=201"
        debug!("Upgrade Server {} to plan {}", sub_id, plan_id);
        let params = &[("SUBID", Some(sub_id)), ("VPSPLANID", Some(plan_id))];
        let body = serde_urlencoded::to_string(params).unwrap();
        RequestBuilder {
            method: Method::POST,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/server/upgrade_plan"),
            resp_t: PhantomData,
            body: Some(body),
        }
    }

    pub fn upgrade_plan_list(self, sub_id: &str) -> RequestBuilder<'t, response::PlanIds> {
        // POST: "https://api.vultr.com/v1/server/upgrade_plan_list"
        // body: "SUBID=576965"
        debug!("Retrieve a list of plan ids for Server {}", sub_id);
        let params = &[("SUBID", Some(sub_id))];
        let url_params = serde_urlencoded::to_string(params).unwrap();
        RequestBuilder {
            method: Method::GET,
            api_key: self.api_key,
            url: format!("https://api.vultr.com/v1/server/upgrade_plan_list?{}", url_params),
            resp_t: PhantomData,
            body: None,
        }
    }

    pub fn backup_enable(self, sub_id: &str) -> RequestBuilder<'t, response::HeaderOnly> {
        // POST: "https://api.vultr.com/v1/server/backup_enable"
        // body: "SUBID=576965"
        debug!("Enable Backup on Server: {}", sub_id);
        RequestBuilder {
            method: Method::POST,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/server/backup_enable"),
            resp_t: PhantomData,
            body: Some(format!("SUBID={}", sub_id)),
        }
    }

    pub fn backup_disable(self, sub_id: &str) -> RequestBuilder<'t, response::HeaderOnly> {
        // POST: "https://api.vultr.com/v1/server/backup_disable"
        // body: "SUBID=576965"
        debug!("Disable Backup on Server: {}", sub_id);
        RequestBuilder {
            method: Method::POST,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/server/backup_disable"),
            resp_t: PhantomData,
            body: Some(format!("SUBID={}", sub_id)),
        }
    }

    pub fn backup_get_schedule(self, sub_id: &str) -> RequestBuilder<'t, response::Schedule> {
        // POST: "https://api.vultr.com/v1/server/backup_get_schedule"
        // body: "SUBID=576965"
        debug!("Get Backup Schedule for Server: {}", sub_id);
        RequestBuilder {
            method: Method::POST,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/server/backup_get_schedule"),
            resp_t: PhantomData,
            body: Some(format!("SUBID={}", sub_id)),
        }
    }

    pub fn backup_set_schedule(self, schedule_opt: &ScheduleOptions) ->
    RequestBuilder<'t, response::HeaderOnly>
    {
        // POST: "https://api.vultr.com/v1/server/backup_set_schedule"
        // body: "SUBID=576965&cron_type=weekly&hour=8&dow=6"
        debug!("Set Backup Schedule for Server: {}", schedule_opt.sub_id);
        RequestBuilder {
            method: Method::POST,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/server/backup_set_schedule"),
            resp_t: PhantomData,
            body: Some(serde_urlencoded::to_string(schedule_opt).unwrap()),
        }
    }

    pub fn restore_backup(self, sub_id: &str, backup_id: &str) ->
    RequestBuilder<'t, response::HeaderOnly>
    {
        // POST: "https://api.vultr.com/v1/server/restore_backup"
        // body: "SUBID=576965&BACKUPID=543d34149403a"
        debug!("Set Backup Schedule for Server: {}", sub_id);
        RequestBuilder {
            method: Method::POST,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/server/backup_set_schedule"),
            resp_t: PhantomData,
            body: Some(format!("SUBID={}&BACKUPID={}", sub_id, backup_id)),
        }
    }

    pub fn label_set(self, sub_id: &str, label: &str) ->
    RequestBuilder<'t, response::HeaderOnly>
    {
        // POST: "https://api.vultr.com/v1/server/label_set"
        // body: "SUBID=576965&label=example"
        debug!("Set Label {} for Server: {}", label, sub_id);
        let params = &[("SUBID", Some(sub_id)), ("label", Some(label))];
        let body = serde_urlencoded::to_string(params).unwrap();
        RequestBuilder {
            method: Method::POST,
            api_key: self.api_key,
            url: String::from("https://api.vultr.com/v1/server/label_set"),
            resp_t: PhantomData,
            body: Some(body),
        }

    }
}

impl<'t> VultrRequest<response::Servers> for RequestBuilder<'t, response::Servers> {}

impl<'t> VultrRequest<response::PlanIds> for RequestBuilder<'t, response::PlanIds> {}
