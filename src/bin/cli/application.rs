use clap::ArgMatches;

use vultrapi::{VultrMgr, request::VultrRequest};

use crate::config::Config;


pub async fn run<'a, 'b>(_m: &'a ArgMatches<'a>, cfg: &'b mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match vultr_mgr.applications().retrieve().await {
        Ok(m_apps) => {
            for (i, app) in &m_apps {
                println!("App {}: {{\n{}\n}}", i, app);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
