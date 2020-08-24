use clap::ArgMatches;

use vultrapi::{VultrMgr, request::VultrRequest};

use crate::config::Config;


pub async fn run<'a, 'b>(_m: &'a ArgMatches<'a>, cfg: &'b mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match vultr_mgr.operating_systems().retrieve().await {
        Ok(m_ops) => {
            for (i, op) in &m_ops {
                println!("Operating System {}: {{\n{}\n}}", i, op);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
