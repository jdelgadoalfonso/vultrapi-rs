use response::{NotArray, NamedResponse};

use std::borrow::Cow;
use std::fmt;


#[derive(Deserialize, Debug)]
pub struct Auth {
    pub acls: Vec<String>,
    pub email: String,
    pub name: String,
}

impl NotArray for Auth {}

impl fmt::Display for Auth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "\tAcls: {:?}\n\
                \tEmail: \"{}\"\n\
                \tName: \"{}\"",
               self.acls,
               self.email,
               self.name)
    }
}

impl NamedResponse for Auth {
    fn name<'a>() -> Cow<'a, str> { "auth".into() }
}
