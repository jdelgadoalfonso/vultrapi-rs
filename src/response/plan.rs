use crate::response::{NotArray, NamedResponse};
use std::{borrow::Cow, collections::HashMap, fmt};


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Plan {
    #[serde(rename="VPSPLANID")]
    pub vps_plan_id: String,
    pub name: String,
    pub vcpu_count: String,
    pub ram: String,
    pub disk: String,
    pub bandwidth: String,
    pub price_per_month: String,
    pub windows: bool,
    pub plan_type: String,
    pub available_locations: Vec<u32>,
    pub deprecated: Option<bool>,
}

impl NotArray for Plan {}

impl fmt::Display for Plan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "\tVPSPLANID: \"{}\"\n\
                \tName: \"{}\"\n\
                \tVCPU Count: \"{}\"\n\
                \tRAM: \"{}\"\n\
                \tDisk: \"{}\"\n\
                \tBandwidth: \"{}\"\n\
                \tPrice per month: \"{}\"\n\
                \tWindows: {}\n\
                \tPlan Type: \"{}\"\n\
                \tAvailable Locations: {:?}\n\
                \tDeprecated: {:?}",
               self.vps_plan_id,
               self.name,
               self.vcpu_count,
               self.ram,
               self.disk,
               self.bandwidth,
               self.price_per_month,
               self.windows,
               self.plan_type,
               self.available_locations,
               self.deprecated)
    }
}

impl NamedResponse for Plan {
    fn name<'a>() -> Cow<'a, str> { "plan".into() }
}

pub type Plans = HashMap<String, Plan>;

impl NamedResponse for Plans {
    fn name<'a>() -> Cow<'a, str> { "plans".into() }
}
