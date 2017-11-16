use clap::ArgMatches;

use vultrapi::VultrMgr;
use vultrapi::request::VultrRequest;

use config::Config;


pub fn run(_m: &ArgMatches, cfg: &mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match vultr_mgr.plans().retrieve() {
        Ok(m_plans) => {
            for (i, plan) in &m_plans {
                println!("Plan {}: {{\n{}\n}}", i, plan);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
