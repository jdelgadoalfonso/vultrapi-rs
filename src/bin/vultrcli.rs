use clap::{App, ArgMatches, SubCommand};
use cli::{account, application, auth, backup, os, plan, region, snapshot, server};
use config::Config;

mod config;
mod cli;


fn get_api_key(m: &ArgMatches) -> String {
    let api_key = if let Some(api_key) = m.value_of("apikey") {
        api_key.to_owned()
    } else {
        "".to_owned()
    };
    if api_key.len() != 36 {
        println!("No Vultr API Key found.\n\n\
        Use `vultrcli --apikey <apikey>` and try again");
        std::process::exit(1);
    }
    api_key
}

#[tokio::main]
pub async fn main() -> () {
    let matches = App::new("vultrcli")
        .version("1.0")
        .author("Jose A. Delgado <jose.delgado@galgus.net>")
        .about("Vultr REST API client command-line utility")
        .args_from_usage("-a, --apikey=[apikey] 'Vultr API Key'")
        .subcommand(SubCommand::with_name("account")
            .about("Retrieve information about the current account"))
        .subcommand(SubCommand::with_name("applications")
            .about("Retrieve a list of available applications. \
                    These refer to applications that can be launched when creating a Vultr VPS"))
        .subcommand(SubCommand::with_name("auth")
            .about("Retrieve information about the current API key"))
        .subcommand(SubCommand::with_name("backup")
            .about("List all backups on the current account"))
        .subcommand(SubCommand::with_name("os")
            .about("Retrieve a list of available operating systems"))
        .subcommand(SubCommand::with_name("plan")
            .about("Retrieve a list of all active plans"))
        .subcommand(SubCommand::with_name("region")
            .about("Retrieve a list of all active regions"))
        .subcommand(SubCommand::with_name("snapshot")
            .about("Handle Snapshots")
            .subcommand(SubCommand::with_name("list")
                .about("List all snapshots on the current account (default)"))
            .subcommand(SubCommand::with_name("create")
                .about("Create a snapshot from an existing virtual machine")
                .arg_from_usage("<id> 'The server ID'")
                .arg_from_usage("[description] 'Optional description'"))
            .subcommand(SubCommand::with_name("destroy")
                .about("Destroy (delete) a snapshot")
                .arg_from_usage("<id> 'The snapshot ID'")))
        .subcommand(SubCommand::with_name("server")
            .about("Handle Servers")
            .subcommand(SubCommand::with_name("list")
                .about("List all active or pending virtual machines on the current account (default)")
                .arg_from_usage("[id] 'The server ID'")
                .arg_from_usage("-l, --loc=[loc] 'The server loc'")
                .arg_from_usage("-t, --tag=[tag] 'The server tag'"))
            .subcommand(SubCommand::with_name("create")
                .about("Create a new virtual machine")
                .arg_from_usage("<location> 'The location ID'")
                .arg_from_usage("<plan> 'The plan ID'")
                .arg_from_usage("<snapshot> 'The snapshot ID'")
                .arg_from_usage("-h, --hostname=[hostname] 'Optional hostname'")
                .arg_from_usage("-l, --label=[label] 'Optional label'")
                .arg_from_usage("-e, --epn=[label] 'Optional label'"))
            .subcommand(SubCommand::with_name("destroy")
                .about("Destroy (delete) a virtual machine")
                .arg_from_usage("<id> 'The server ID'"))
            .subcommand(SubCommand::with_name("halt")
                .about("Halt a virtual machine")
                .arg_from_usage("<id> 'The server ID'"))
            .subcommand(SubCommand::with_name("start")
                .about("Start a virtual machine")
                .arg_from_usage("<id> 'The server ID'"))
            .subcommand(SubCommand::with_name("reboot")
                .about("Reboot a virtual machine")
                .arg_from_usage("<id> 'The snapshot ID'"))
            .subcommand(SubCommand::with_name("upgrade_plan")
                .about("Upgrade the plan of a virtual machine. The virtual machine will be rebooted upon a successful upgrade.")
                .arg_from_usage("<id> 'The server ID'")
                .arg_from_usage("<plan> 'The new plan'"))
            .subcommand(SubCommand::with_name("backup")
                .about("Enable or Disable Backup Schedule in a server")
                .arg_from_usage("<id> 'The server ID'")
                .arg_from_usage("<enable> 'enable or disable backup (true/false)'"))
            .subcommand(SubCommand::with_name("schedule")
                .about("Schedule Backups in a server")
                .subcommand(SubCommand::with_name("get")
                    .about("Retrieves the backup schedule for a server")
                    .arg_from_usage("<id> 'The server ID'"))
                .subcommand(SubCommand::with_name("set")
                    .about("Sets the backup schedule for a server")
                    .arg_from_usage("<id> 'The server ID'")
                    .arg_from_usage("<crontype> 'Can be one of \'daily\', \'weekly\', \'monthly\', \'daily_alt_even\', or \'daily_alt_odd\''")
                    .arg_from_usage("-h, --hour=[hour] 'Hour value (0-23)'")
                    .arg_from_usage("-w, --dow=[dow] 'Day-of-week value (0-6). Applicable to crons: \'weekly\''")
                    .arg_from_usage("-m, --dom=[dom] 'Day-of-month value (1-28). Applicable to crons: \'monthly\''"))))
        .get_matches();

    let mut cfg = Config {
        api_key: get_api_key(&matches),
    };

    match matches.subcommand() {
        ("account", Some(m)) => account::run(m, &mut cfg).await,
        ("applications", Some(m)) => application::run(m, &mut cfg).await,
        ("auth", Some(m)) => auth::run(m, &mut cfg).await,
        ("backup", Some(m)) => backup::run(m, &mut cfg).await,
        ("os", Some(m)) => os::run(m, &mut cfg).await,
        ("plan", Some(m)) => plan::run(m, &mut cfg).await,
        ("region", Some(m)) => region::run(m, &mut cfg).await,
        ("snapshot", Some(m)) => snapshot::run(m, &mut cfg).await,
        ("server", Some(m)) => server::run(m, &mut cfg).await,
        _ => (),
    }
}
