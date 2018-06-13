use response::{NotArray, NamedResponse};

use std::borrow::Cow;
use std::fmt;


#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub balance: String,
    pub pending_charges: String,
    pub last_payment_date: String,
    pub last_payment_amount: String,
}

impl NotArray for Account {}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "\tBalance: \"{}\"\n\
                \tPending Charges: \"{}\"\n\
                \tLast Payment Date: \"{}\"\n\
                \tLast Payment Amount: \"{}\"",
               self.balance,
               self.pending_charges,
               self.last_payment_date,
               self.last_payment_amount)
    }
}

impl NamedResponse for Account {
    fn name<'a>() -> Cow<'a, str> { "account".into() }
}
