use clap::ArgMatches;

use vultrapi::VultrMgr;
use vultrapi::request::VultrRequest;

use config::Config;


fn list_snapshots(vultr_mgr: &VultrMgr) {
    match vultr_mgr.snapshots().retrieve() {
        Ok(m_snapshots) => {
            for (i, snapshot) in &m_snapshots {
                println!("Snapshot {}: {{\n{}\n}}", i, snapshot);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}

fn create_snapshot(m: &ArgMatches, vultr_mgr: &VultrMgr) {
    let id = m.value_of("id").unwrap();
    let description = m.value_of("description");
    match vultr_mgr.snapshots().create(id, description).retrieve() {
        Ok(created_snpsh) => println!("Created Snapshot: {{\n{}\n}}", created_snpsh),
        Err(e) => println!("Error: {}", e),
    }
}

fn destroy_snapshot(m: &ArgMatches, vultr_mgr: &VultrMgr) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.snapshots().destroy(id).retrieve() {
        Ok(status) => println!("Destroy Snapshot: {}", status),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn run(m: &ArgMatches, cfg: &mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match m.subcommand() {
        ("list", Some(_)) => list_snapshots(&vultr_mgr),
        ("create", Some(m)) => create_snapshot(m, &vultr_mgr),
        ("destroy", Some(m)) => destroy_snapshot(m, &vultr_mgr),
        _ => list_snapshots(&vultr_mgr),
    }
}
