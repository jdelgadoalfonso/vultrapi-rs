use clap::ArgMatches;

use vultrapi::VultrMgr;
use vultrapi::request::VultrRequest;

use config::Config;


pub fn run(_m: &ArgMatches, cfg: &mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match vultr_mgr.operating_systems().retrieve() {
        Ok(m_ops) => {
            for (i, op) in &m_ops {
                println!("Operating System {}: {{\n{}\n}}", i, op);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
