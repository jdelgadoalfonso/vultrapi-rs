use crate::response::{NotArray, NamedResponse};

use std::borrow::Cow;
use std::fmt;
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Application {
    #[serde(rename="APPID")]
    pub app_id: String,
    pub name: String,
    pub short_name: String,
    pub deploy_name: String,
    pub surcharge: u32,
}

impl NotArray for Application {}

impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "\tAPPID: \"{}\"\n\
                \tName: \"{}\"\n\
                \tShort Name: \"{}\"\n\
                \tDeploy Name: \"{}\"\n\
                \tSurcharge: {}",
               self.app_id,
               self.name,
               self.short_name,
               self.deploy_name,
               self.surcharge)
    }
}

impl NamedResponse for Application {
    fn name<'a>() -> Cow<'a, str> { "application".into() }
}

pub type Applications = HashMap<String, Application>;

impl NamedResponse for Applications {
    fn name<'a>() -> Cow<'a, str> { "applications".into() }
}
