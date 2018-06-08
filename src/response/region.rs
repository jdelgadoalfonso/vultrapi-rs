#![allow(non_snake_case)]

use response::{NotArray, NamedResponse};
use std::{borrow::Cow, collections::HashMap, fmt};


#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Region {
    #[serde(rename="DCID")]
    pub dc_id: String,
    pub name: String,
    pub country: String,
    pub continent: String,
    pub state: String,
    pub ddos_protection: bool,
    pub block_storage: bool,
    pub regioncode: String,
}

impl NotArray for Region {}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "\tDCID: \"{}\"\n\
                \tName: \"{}\"\n\
                \tCountry: \"{}\"\n\
                \tContinent: \"{}\"\n\
                \tState: \"{}\"\n\
                \tDDOS Protection: \"{}\"\n\
                \tBlock Storage: \"{}\"\n\
                \tRegion Code: \"{}\"",
               self.dc_id,
               self.name,
               self.country,
               self.continent,
               self.state,
               self.ddos_protection,
               self.block_storage,
               self.regioncode)
    }
}

impl NamedResponse for Region {
    fn name<'a>() -> Cow<'a, str> { "region".into() }
}

pub type Regions = HashMap<String, Region>;

impl NamedResponse for Regions {
    fn name<'a>() -> Cow<'a, str> { "regions".into() }
}
