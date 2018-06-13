use response::{NotArray, NamedResponse};
use std::{borrow::Cow, fmt};


#[derive(Serialize, Deserialize, Debug)]
pub struct Schedule {
    pub enabled: bool,
    pub cron_type: String,
    pub next_scheduled_time_utc: String,
    pub hour: u32,
    pub dow: u32,
    pub dom: u32,
}

impl NotArray for Schedule {}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "\tEnabled: {}\n\
                \tCron Type: \"{}\"\n\
                \tNext Scheduled Time UTC: \"{}\"\n\
                \tHour: {}\n\
                \tDow: {}\n\
                \tDom: {}",
               self.enabled,
               self.cron_type,
               self.next_scheduled_time_utc,
               self.hour,
               self.dow,
               self.dom)
    }
}

impl NamedResponse for Schedule {
    fn name<'a>() -> Cow<'a, str> { "schedule".into() }
}
