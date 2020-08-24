use crate::response::{NotArray, NamedResponse};
use std::{borrow::Cow, collections::HashMap, fmt};


#[derive(Serialize, Deserialize, Debug)]
pub struct CreatedSnapshot {
    #[serde(rename="SNAPSHOTID")]
    pub snapshot_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Snapshot {
    #[serde(rename="SNAPSHOTID")]
    pub snapshot_id: String,
    pub date_created: String,
    pub description: String,
    pub size: String,
    pub status: String,
}

impl NotArray for CreatedSnapshot {}

impl NotArray for Snapshot {}

impl fmt::Display for CreatedSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SNAPSHOTID: \"{}\"", self.snapshot_id)
    }
}

impl fmt::Display for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "\tSNAPSHOTID: \"{}\"\n\
                \tDate Created: \"{}\"\n\
                \tDescription: \"{}\"\n\
                \tSize: \"{}\"\n\
                \tStatus: {}",
               self.snapshot_id,
               self.date_created,
               self.description,
               self.size,
               self.status)
    }
}

impl NamedResponse for CreatedSnapshot {
    fn name<'a>() -> Cow<'a, str> { "created snapshot".into() }
}

impl NamedResponse for Snapshot {
    fn name<'a>() -> Cow<'a, str> { "snapshot".into() }
}

pub type Snapshots = HashMap<String, Snapshot>;

impl NamedResponse for Snapshots {
    fn name<'a>() -> Cow<'a, str> { "snapshots".into() }
}
