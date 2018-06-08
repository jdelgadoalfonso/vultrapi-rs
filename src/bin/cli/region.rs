use clap::ArgMatches;
use vultrapi::{VultrMgr, request::VultrRequest};
use config::Config;


pub fn run(_m: &ArgMatches, cfg: &mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match vultr_mgr.regions().retrieve() {
        Ok(m_regs) => {
            for (i, reg) in &m_regs {
                println!("Region {}: {{\n{}\n}}", i, reg);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
