use clap::ArgMatches;

use vultrapi::{VultrMgr, request::VultrRequest};

use crate::config::Config;


async fn list_snapshots<'a>(vultr_mgr: &'a VultrMgr<'a>) {
    match vultr_mgr.snapshots().retrieve().await {
        Ok(m_snapshots) => {
            for (i, snapshot) in &m_snapshots {
                println!("Snapshot {}: {{\n{}\n}}", i, snapshot);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}

async fn create_snapshot<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    let id = m.value_of("id").unwrap();
    let description = m.value_of("description");
    match vultr_mgr.snapshots().create(id, description).retrieve().await {
        Ok(created_snpsh) => println!("Created Snapshot: {{\n{}\n}}", created_snpsh),
        Err(e) => println!("Error: {}", e),
    }
}

async fn destroy_snapshot<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.snapshots().destroy(id).retrieve().await {
        Ok(status) => println!("Destroy Snapshot: {}", status),
        Err(e) => println!("Error: {}", e),
    }
}

pub async fn run<'t, 'a>(m: &'t ArgMatches<'a>, cfg: &'t Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match m.subcommand() {
        ("list", Some(_)) => list_snapshots(&vultr_mgr).await,
        ("create", Some(m)) => create_snapshot(m, &vultr_mgr).await,
        ("destroy", Some(m)) => destroy_snapshot(m, &vultr_mgr).await,
        _ => list_snapshots(&vultr_mgr).await,
    }
}
