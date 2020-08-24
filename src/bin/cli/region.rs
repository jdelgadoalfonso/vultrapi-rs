use clap::ArgMatches;

use vultrapi::{VultrMgr, request::VultrRequest};

use crate::config::Config;


pub async fn run<'a, 'b>(_m: &'a ArgMatches<'a>, cfg: &'b mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match vultr_mgr.regions().retrieve().await {
        Ok(m_regs) => {
            for (i, reg) in &m_regs {
                println!("Region {}: {{\n{}\n}}", i, reg);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
