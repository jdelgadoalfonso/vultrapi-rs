use clap::ArgMatches;

use vultrapi::{VultrMgr, request::VultrRequest};

use crate::config::Config;


pub async fn run<'a, 'b>(_m: &'a ArgMatches<'a>, cfg: &'b mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match vultr_mgr.plans().retrieve().await {
        Ok(m_plans) => {
            for (i, plan) in &m_plans {
                println!("Plan {}: {{\n{}\n}}", i, plan);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
