use clap::ArgMatches;

use vultrapi::{VultrMgr, request::VultrRequest};

use crate::config::Config;


pub async fn run<'a, 'b>(_m: &'a ArgMatches<'a>, cfg: &'b mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match vultr_mgr.backups().retrieve().await {
        Ok(m_backups) => {
            for (i, backup) in &m_backups {
                println!("Backup {}: {{\n{}\n}}", i, backup);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
