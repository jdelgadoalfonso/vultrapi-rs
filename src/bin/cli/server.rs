use clap::ArgMatches;
use vultrapi::{VultrMgr, ServerOptions, ScheduleOptions, request::VultrRequest};
use config::Config;


fn get_str_to_u32(param: Option<&str>) -> Option<u32> {
    if let Some(p) = param {
        if let Ok(n) = p.parse::<u32>() {
            return Some(n);
        }
    }
    None
}

fn list_servers(m: &ArgMatches, vultr_mgr: &VultrMgr) {
    if let Some(sub_id) = m.value_of("id") {
        match vultr_mgr.server_by_filter(sub_id).retrieve() {
            Ok(server) => println!("Server: {{\n{}\n}}", server),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        let location = m.value_of("loc").unwrap_or("");
        let tag = m.value_of("tag").unwrap_or("");
        match vultr_mgr.servers().retrieve() {
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

fn create_server(m: &ArgMatches, vultr_mgr: &VultrMgr) {
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
        hostname: hostname,
        label: label,
        tag: None,
        enable_private_network: epn,
    }).retrieve() {
        Ok(server) => println!("New Server: {}", server),
        Err(e) => println!("Error: {}", e),
    }
}

fn destroy_server(m: &ArgMatches, vultr_mgr: &VultrMgr) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.snapshots().destroy(id).retrieve() {
        Ok(status) => println!("Destroy Snapshot: {}", status),
        Err(e) => println!("Error: {}", e),
    }
}

fn reboot_server(m: &ArgMatches, vultr_mgr: &VultrMgr) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.servers().reboot(id).retrieve() {
        Ok(header_res) => println!("Server Reboot Status: {}", header_res),
        Err(e) => println!("Error: {}", e),
    }
}

fn halt_server(m: &ArgMatches, vultr_mgr: &VultrMgr) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.servers().halt(id).retrieve() {
        Ok(header_res) => println!("Server Halt Status: {}", header_res),
        Err(e) => println!("Error: {}", e),
    }
}

fn start_server(m: &ArgMatches, vultr_mgr: &VultrMgr) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.servers().start(id).retrieve() {
        Ok(header_res) => println!("Server Start Status: {}", header_res),
        Err(e) => println!("Error: {}", e),
    }
}

fn upgrade_plan(m: &ArgMatches, vultr_mgr: &VultrMgr) {
    let id = m.value_of("id").unwrap();
    let plan = m.value_of("plan").unwrap();
    match vultr_mgr.servers().upgrade_plan(id, plan).retrieve() {
        Ok(header_res) => println!("Upgrade Server Plan Status: {}", header_res),
        Err(e) => println!("Error: {}", e),
    }
}

fn backup(m: &ArgMatches, vultr_mgr: &VultrMgr) {
    match m.value_of("enable") {
        Some("true") => backup_enable(m, &vultr_mgr),
        Some("false") => backup_disable(m, &vultr_mgr),
        _ => println!("<enable> is a bool parameter."),
    }
}

fn backup_enable(m: &ArgMatches, vultr_mgr: &VultrMgr) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.servers().backup_enable(id).retrieve() {
        Ok(header_res) => println!("Server Backup Enable: {}", header_res),
        Err(e) => println!("Error: {}", e),
    }
}

fn backup_disable(m: &ArgMatches, vultr_mgr: &VultrMgr) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.servers().backup_disable(id).retrieve() {
        Ok(header_res) => println!("Server Backup Disable: {}", header_res),
        Err(e) => println!("Error: {}", e),
    }
}

fn schedule(m: &ArgMatches, vultr_mgr: &VultrMgr) {
    match m.subcommand() {
        ("get", Some(m)) => backup_get_schedule(m, &vultr_mgr),
        ("set", Some(m)) => backup_set_schedule(m, &vultr_mgr),
        _ => (),
    }
}

fn backup_get_schedule(m: &ArgMatches, vultr_mgr: &VultrMgr) {
    let id = m.value_of("id").unwrap();
    match vultr_mgr.servers().backup_get_schedule(id).retrieve() {
        Ok(schedule) => println!("Server Backup Schedule: {{\n{}\n}}", schedule),
        Err(e) => println!("Error: {}", e),
    }
}

fn backup_set_schedule(m: &ArgMatches, vultr_mgr: &VultrMgr) {
    let sub_id = m.value_of("id").unwrap();
    let cron_type = m.value_of("crontype").unwrap();
    let hour = get_str_to_u32(m.value_of("hour"));
    let dow = get_str_to_u32(m.value_of("dow"));
    let dom = get_str_to_u32(m.value_of("dom"));

    match vultr_mgr.servers().backup_set_schedule(&ScheduleOptions {
        sub_id: sub_id,
        cron_type: cron_type,
        hour: hour,
        dow: dow,
        dom: dom,
    }).retrieve() {
        Ok(status) => println!("Server Schedule Set: {}", status),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn run(m: &ArgMatches, cfg: &mut Config) {
    let vultr_mgr = VultrMgr::with_api_key(&cfg.api_key[..]);

    match m.subcommand() {
        ("list", Some(m)) => list_servers(m, &vultr_mgr),
        ("create", Some(m)) => create_server(m, &vultr_mgr),
        ("destroy", Some(m)) => destroy_server(m, &vultr_mgr),
        ("reboot", Some(m)) => reboot_server(m, &vultr_mgr),
        ("halt", Some(m)) => halt_server(m, &vultr_mgr),
        ("start", Some(m)) => start_server(m, &vultr_mgr),
        ("upgrade_plan", Some(m)) => upgrade_plan(m, &vultr_mgr),
        ("backup", Some(m)) => backup(m, &vultr_mgr),
        ("schedule", Some(m)) => schedule(m, &vultr_mgr),
        _ => list_servers(m, &vultr_mgr),
    }
}
