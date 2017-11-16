use clap::ArgMatches;

use vultrapi::VultrMgr;
use vultrapi::request::VultrRequest;

use config::Config;


pub fn run(_m: &ArgMatches, cfg: &mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match vultr_mgr.account().retrieve() {
        Ok(action_res) => println!("Account: {{\n{}\n}}", action_res),
        Err(e)         => println!("Error: {}", e),
    }
}
