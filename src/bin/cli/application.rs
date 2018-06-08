use clap::ArgMatches;
use vultrapi::{VultrMgr, request::VultrRequest};
use config::Config;


pub fn run(_m: &ArgMatches, cfg: &mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match vultr_mgr.applications().retrieve() {
        Ok(m_apps) => {
            for (i, app) in &m_apps {
                println!("App {}: {{\n{}\n}}", i, app);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
