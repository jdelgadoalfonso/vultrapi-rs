use response::{NotArray, NamedResponse};
use std::{borrow::Cow, collections::HashMap, fmt};


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CreatedServer {
    #[serde(rename="SUBID")]
    sub_id: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Server {
    #[serde(rename="SUBID")]
    pub sub_id: String,
    pub os: String,
    pub ram: String,
    pub disk: String,
    pub main_ip: String,
    pub vcpu_count: String,
    pub location: String,
    #[serde(rename="DCID")]
    pub dc_id: String,
    pub default_password: String,
    pub date_created: String,
    pub pending_charges: String,
    pub status: String,
    pub cost_per_month: String,
    pub current_bandwidth_gb: f64,
    pub allowed_bandwidth_gb: String,
    pub netmask_v4: String,
    pub gateway_v4: String,
    pub power_status: String,
    pub server_state: String,
    #[serde(rename="VPSPLANID")]
    pub vps_plan_id: String,
    pub v6_network: String,
    pub v6_main_ip: String,
    pub v6_network_size: String,
    pub v6_networks: Vec<HashMap<String, String>>,
    pub label: String,
    pub internal_ip: String,
    pub kvm_url: String,
    pub auto_backups: String,
    pub tag: String,
    #[serde(rename="OSID")]
    pub os_id: String,
    #[serde(rename="APPID")]
    pub app_id: String,
    #[serde(rename="FIREWALLGROUPID")]
    pub firewall_group_id: String,
}

impl NotArray for CreatedServer {}

impl NotArray for Server {}

impl fmt::Display for CreatedServer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SUBID: \"{}\"", self.sub_id)
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "\tSUBID: \"{}\"\n\
                \tOS: \"{}\"\n\
                \tRam: \"{}\"\n\
                \tDisk: \"{}\"\n\
                \tMain IP: \"{}\"\n\
                \tVCPU Count: \"{}\"\n\
                \tLocation: \"{}\"\n\
                \tDCID: \"{}\"\n\
                \tDefault Password: \"{}\"\n\
                \tDate Created: \"{}\"\n\
                \tPending Charges: \"{}\"\n\
                \tStatus: \"{}\"\n\
                \tCost per month: \"{}\"\n\
                \tCurrent Bandwidth: \"{}\" Gb\n\
                \tAllowed Bandwidth: \"{}\" Gb\n\
                \tNetmask v4: \"{}\"\n\
                \tGateway v4: \"{}\"\n\
                \tPower Status: \"{}\"\n\
                \tServer State: \"{}\"\n\
                \tVPSPLANID: \"{}\"\n\
                \tv6 Network: {:?}\n\
                \tv6 Main IP: \"{}\"\n\
                \tv6 Network Size: \"{}\"\n\
                \tLabel: \"{}\"\n\
                \tInternal IP: \"{}\"\n\
                \tKVM URL: \"{}\"\n\
                \tAuto Backups: \"{}\"\n\
                \tTag: \"{}\"\n\
                \tOSID: \"{}\"\n\
                \tAPPID: \"{}\"\n\
                \tFIREWALLGROUPID: \"{}\"",
               self.sub_id,
               self.os,
               self.ram,
               self.disk,
               self.main_ip,
               self.vcpu_count,
               self.location,
               self.dc_id,
               self.default_password,
               self.date_created,
               self.pending_charges,
               self.status,
               self.cost_per_month,
               self.current_bandwidth_gb,
               self.allowed_bandwidth_gb,
               self.netmask_v4,
               self.gateway_v4,
               self.power_status,
               self.server_state,
               self.vps_plan_id,
               self.v6_network,
               self.v6_main_ip,
               self.v6_network_size,
               self.label,
               self.internal_ip,
               self.kvm_url,
               self.auto_backups,
               self.tag,
               self.os_id,
               self.app_id,
               self.firewall_group_id)
    }
}

impl NamedResponse for CreatedServer {
    fn name<'a>() -> Cow<'a, str> { "created server".into() }
}

impl NamedResponse for Server {
    fn name<'a>() -> Cow<'a, str> { "server".into() }
}

pub type Servers = HashMap<String, Server>;

impl NamedResponse for Servers {
    fn name<'a>() -> Cow<'a, str> { "server".into() }
}

pub type PlanIds = Vec<u32>;

impl NamedResponse for PlanIds {
    fn name<'a>() -> Cow<'a, str> { "plan Ids".into() }
}
