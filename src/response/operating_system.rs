#![allow(non_snake_case)]

use response::{NotArray, NamedResponse};
use std::{borrow::Cow, collections::HashMap, fmt};


#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct OperatingSystem {
    pub OSID: u32,
    pub name: String,
    pub arch: String,
    pub family: String,
    pub windows: bool,
}

impl NotArray for OperatingSystem {}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "\tOSID: {}\n\
                \tName: \"{}\"\n\
                \tArchitecture: \"{}\"\n\
                \tFamily: \"{}\"\n\
                \tWindows: {}",
               self.OSID,
               self.name,
               self.arch,
               self.family,
               self.windows)
    }
}

impl NamedResponse for OperatingSystem {
    fn name<'a>() -> Cow<'a, str> { "operating system".into() }
}

pub type OperatingSystems = HashMap<String, OperatingSystem>;

impl NamedResponse for OperatingSystems {
    fn name<'a>() -> Cow<'a, str> { "operating systems".into() }
}
