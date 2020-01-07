#![crate_type = "lib"]

#[macro_use]
extern crate failure;
extern crate hyper;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;

mod vultr_mgr;
#[macro_use]
mod macros;

pub mod request;
pub mod response;

pub use vultr_mgr::VultrMgr;
pub use request::{RequestBuilder, ServerOptions, ScheduleOptions};

pub type ResultVultr<T> = Result<T, failure::Error>;
