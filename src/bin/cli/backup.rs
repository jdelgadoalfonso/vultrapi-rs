use clap::ArgMatches;

use vultrapi::VultrMgr;
use vultrapi::request::VultrRequest;

use config::Config;


pub fn run(_m: &ArgMatches, cfg: &mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match vultr_mgr.backups().retrieve() {
        Ok(m_backups) => {
            for (i, backup) in &m_backups {
                println!("Backup {}: {{\n{}\n}}", i, backup);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
