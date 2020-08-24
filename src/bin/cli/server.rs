use clap::ArgMatches;

use vultrapi::{VultrMgr, ServerOptions, ScheduleOptions, request::VultrRequest};

use crate::config::Config;


fn get_str_to_u32(param: Option<&str>) -> Option<u32> {
    if let Some(p) = param {
        if let Ok(n) = p.parse::<u32>() {
            return Some(n);
        }
    }
    None
}

async fn list_servers<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    if let Some(sub_id) = m.value_of("id") {
        match vultr_mgr.server_by_filter(sub_id).retrieve().await {
            Ok(server) => println!("Server: {{\n{}\n}}", server),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        let location = m.value_of("loc").unwrap_or("");
        let tag = m.value_of("tag").unwrap_or("");
        match vultr_mgr.servers().retrieve().await {
            Ok(m_servers) => {
                for (i, server) in m_servers.iter()
                    .filter(|(_, v)| (location == "" && tag == "") ||
                                     (v.location == location && v.tag == tag)) {
                    println!("Server {}: {{\n{}\n}}", i, server);
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    };
}

async fn create_server<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    let location = m.value_of("location").unwrap();
    let plan = m.value_of("plan").unwrap();
    let snapshot = m.value_of("snapshot").unwrap();
    let hostname = m.value_of("hostname");
    let label = m.value_of("label");
    let epn = if m.occurrences_of("epn") > 0 { Some(true) } else { None };

    match vultr_mgr.servers().create(&ServerOptions {
        dc_id: location,
        vps_plan_id: plan,
        os_id: "164",
        snapshot_id: Some(snapshot),
        hostname,
        label,
        tag: None,
        enable_private_network: epn,
    }).retrieve().await {
        Ok(server) => println!("New Server: {}", server),
        Err(e) => println!("Error: {}", e),
    }
}

async fn destroy_server<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.snapshots().destroy(id).retrieve().await {
        Ok(status) => println!("Destroy Snapshot: {}", status),
        Err(e) => println!("Error: {}", e),
    }
}

async fn reboot_server<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.servers().reboot(id).retrieve().await {
        Ok(header_res) => println!("Server Reboot Status: {}", header_res),
        Err(e) => println!("Error: {}", e),
    }
}

async fn halt_server<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.servers().halt(id).retrieve().await {
        Ok(header_res) => println!("Server Halt Status: {}", header_res),
        Err(e) => println!("Error: {}", e),
    }
}

async fn start_server<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.servers().start(id).retrieve().await {
        Ok(header_res) => println!("Server Start Status: {}", header_res),
        Err(e) => println!("Error: {}", e),
    }
}

async fn upgrade_plan<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    let id = m.value_of("id").unwrap();
    let plan = m.value_of("plan").unwrap();
    match vultr_mgr.servers().upgrade_plan(id, plan).retrieve().await {
        Ok(header_res) => println!("Upgrade Server Plan Status: {}", header_res),
        Err(e) => println!("Error: {}", e),
    }
}

async fn backup<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    match m.value_of("enable") {
        Some("true") => backup_enable(m, &vultr_mgr).await,
        Some("false") => backup_disable(m, &vultr_mgr).await,
        _ => println!("<enable> is a bool parameter."),
    }
}

async fn backup_enable<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.servers().backup_enable(id).retrieve().await {
        Ok(header_res) => println!("Server Backup Enable: {}", header_res),
        Err(e) => println!("Error: {}", e),
    }
}

async fn backup_disable<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.servers().backup_disable(id).retrieve().await {
        Ok(header_res) => println!("Server Backup Disable: {}", header_res),
        Err(e) => println!("Error: {}", e),
    }
}

async fn schedule<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    match m.subcommand() {
        ("get", Some(m)) => backup_get_schedule(m, &vultr_mgr).await,
        ("set", Some(m)) => backup_set_schedule(m, &vultr_mgr).await,
        _ => (),
    }
}

async fn backup_get_schedule<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.servers().backup_get_schedule(id).retrieve().await {
        Ok(schedule) => println!("Server Backup Schedule: {{\n{}\n}}", schedule),
        Err(e) => println!("Error: {}", e),
    }
}

async fn backup_set_schedule<'a, 'b>(m: &'a ArgMatches<'a>, vultr_mgr: &'b VultrMgr<'b>) {
    let sub_id = m.value_of("id").unwrap();
    let cron_type = m.value_of("crontype").unwrap();
    let hour = get_str_to_u32(m.value_of("hour"));
    let dow = get_str_to_u32(m.value_of("dow"));
    let dom = get_str_to_u32(m.value_of("dom"));

    match vultr_mgr.servers().backup_set_schedule(&ScheduleOptions {
        sub_id,
        cron_type,
        hour,
        dow,
        dom,
    }).retrieve().await {
        Ok(status) => println!("Server Schedule Set: {}", status),
        Err(e) => println!("Error: {}", e),
    }
}

pub async fn run<'a, 'b>(m: &'a ArgMatches<'a>, cfg: &'b mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match m.subcommand() {
        ("list", Some(m)) => list_servers(m, &vultr_mgr).await,
        ("create", Some(m)) => create_server(m, &vultr_mgr).await,
        ("destroy", Some(m)) => destroy_server(m, &vultr_mgr).await,
        ("reboot", Some(m)) => reboot_server(m, &vultr_mgr).await,
        ("halt", Some(m)) => halt_server(m, &vultr_mgr).await,
        ("start", Some(m)) => start_server(m, &vultr_mgr).await,
        ("upgrade_plan", Some(m)) => upgrade_plan(m, &vultr_mgr).await,
        ("backup", Some(m)) => backup(m, &vultr_mgr).await,
        ("schedule", Some(m)) => schedule(m, &vultr_mgr).await,
        _ => list_servers(m, &vultr_mgr).await,
    }
}
