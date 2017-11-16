#![crate_type= "lib"]
#![feature(custom_derive,
           custom_attribute,
           plugin)]
#![deny(trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unused_import_braces)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;

mod vultr_mgr;

pub mod request;
pub mod response;

pub use vultr_mgr::VultrMgr;
pub use request::RequestBuilder;
