#![allow(non_snake_case)]

use response::{NotArray, NamedResponse};

use std::borrow::Cow;
use std::fmt;
use std::collections::HashMap;


#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Backup {
    pub BACKUPID: String,
    pub date_created: String,
    pub description: String,
    pub size: String,
    pub status: String,
}

impl NotArray for Backup {}

impl fmt::Display for Backup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "\tBACKUPID: \"{}\"\n\
                \tDate Created: \"{}\"\n\
                \tDescription: \"{}\"\n\
                \tSize: \"{}\"\n\
                \tStatus: {}",
               self.BACKUPID,
               self.date_created,
               self.description,
               self.size,
               self.status)
    }
}

impl NamedResponse for Backup {
    fn name<'a>() -> Cow<'a, str> { "backup".into() }
}

pub type Backups = HashMap<String, Backup>;

impl NamedResponse for Backups {
    fn name<'a>() -> Cow<'a, str> { "backups".into() }
}
