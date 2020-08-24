use clap::ArgMatches;

use vultrapi::{VultrMgr, request::VultrRequest};

use crate::config::Config;


pub async fn run<'a, 'b>(_m: &'a ArgMatches<'a>, cfg: &'b mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match vultr_mgr.auth().retrieve().await {
        Ok(auth_res) => println!("Auth: {{\n{}\n}}", auth_res),
        Err(e) => println!("Error: {}", e),
    }
}
