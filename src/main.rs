mod api;
mod commands;
mod config;
mod detached;
mod jar;
mod java;
mod modpack;
mod profiles;
mod progress;

use clap::{Arg, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn cli() -> Command {
    Command::new("mcvcli")
        .about("A simple CLI for interacting with Minecraft servers")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .version(VERSION)
        .subcommand(
            Command::new("upgrade")
                .about("Upgrades the CLI to the latest version")
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("init")
                .about("Initializes a new Minecraft server")
                .arg(
                    Arg::new("directory")
                        .help("The directory to initialize the server in")
                        .num_args(1)
                        .default_value(".")
                        .required(false),
                )
                .arg(
                    Arg::new("file")
                        .help("The file to initialize the server with (options: install, modrinth, <any jar file>)")
                        .long("file")
                        .short('f')
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("type")
                        .help("The type of server to install (options: https://mcjars.app)")
                        .long("type")
                        .short('t')
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("version")
                        .help("The version of the server type to install")
                        .long("version")
                        .short('v')
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("build")
                        .help("The build id or build name of the server type to install")
                        .long("build")
                        .short('b')
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("ram")
                        .help("The amount of RAM to allocate to the server (in MB)")
                        .long("ram")
                        .short('r')
                        .num_args(1)
                        .value_parser(clap::value_parser!(u16).range(1024..=49152))
                        .required(false),
                )
                .arg(
                    Arg::new("java")
                        .help("The java version to use")
                        .long("java")
                        .short('j')
                        .num_args(1)
                        .value_parser(clap::value_parser!(u8).range(8..=50))
                        .required(false),
                )
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("config")
                .about("Manages the configuration file")
                .arg(
                    Arg::new("profile")
                        .long("profile")
                        .short('p')
                        .help("The profile to use")
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("ram")
                        .long("ram")
                        .short('r')
                        .help("The amount of RAM to allocate to the server (in MB)")
                        .num_args(1)
                        .value_parser(clap::value_parser!(u16).range(1024..=49152))
                        .required(false),
                )
                .arg(
                    Arg::new("stop_command")
                        .long("stop-command")
                        .short('s')
                        .help("The stop command to use when stopping the server")
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("flags")
                        .long("flags")
                        .short('f')
                        .help("The extra flags to pass to the server when starting")
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("args")
                        .long("args")
                        .short('a')
                        .help("The extra args to pass to the server when starting")
                        .num_args(1)
                        .required(false),
                )
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("install")
                .about("Install a new version of the Minecraft server")
                .arg(
                    Arg::new("wipe")
                        .long("wipe")
                        .short('w')
                        .help("Wipe the server directory before installing")
                        .num_args(0)
                        .default_value("false")
                        .value_parser(clap::value_parser!(bool))
                        .required(false),
                )
                .arg(
                    Arg::new("file")
                        .help("The file to initialize the server with (options: install, modrinth, <any jar file>)")
                        .long("file")
                        .short('f')
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("type")
                        .help("The type of server to install (options: https://mcjars.app)")
                        .long("type")
                        .short('t')
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("version")
                        .help("The version of the server type to install")
                        .long("version")
                        .short('v')
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("build")
                        .help("The build id or build name of the server type to install")
                        .long("build")
                        .short('b')
                        .num_args(1)
                        .required(false),
                )
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("start")
                .about("Starts the Minecraft server")
                .arg(
                    Arg::new("eula")
                        .long("eula")
                        .short('e')
                        .help("Accept the Minecraft EULA automatically")
                        .num_args(0)
                        .default_value("false")
                        .value_parser(clap::value_parser!(bool))
                        .required(false),
                )
                .arg(
                    Arg::new("detached")
                        .long("detached")
                        .short('d')
                        .help("Run the server in detached mode (background)")
                        .num_args(0)
                        .default_value("false")
                        .value_parser(clap::value_parser!(bool))
                        .required(false),
                )
                .arg(
                    Arg::new("timeout")
                        .long("timeout")
                        .short('t')
                        .help("The amount of time to wait for the server to stop (seconds)")
                        .num_args(1)
                        .default_value("20")
                        .value_parser(clap::value_parser!(u64).range(1..))
                        .required(false),
                )
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("stop")
                .about("Stops the Minecraft server")
                .arg(
                    Arg::new("timeout")
                        .long("timeout")
                        .short('t')
                        .help("The amount of time to wait for the server to stop (seconds)")
                        .num_args(1)
                        .default_value("20")
                        .value_parser(clap::value_parser!(u64).range(1..))
                        .required(false),
                )
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("attach")
                .about("Attaches to the Minecraft server console")
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("status")
                .about("Gets the status of the Minecraft server (when detached)")
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("lookup")
                .about("Looks up a Player on your server")
                .arg(
                    Arg::new("player")
                        .help("The player to look up")
                        .num_args(1)
                        .required(true),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("query")
                .about("Queries the Minecraft server for information")
                .arg(
                    Arg::new("address")
                        .help("The address of the Minecraft server to query (e.g., `example.com:25565`)")
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("query")
                        .long("query")
                        .short('q')
                        .help("Use the query protocol to get more information (requires server to have query enabled)")
                        .num_args(0)
                        .default_value("false")
                        .value_parser(clap::value_parser!(bool))
                        .required(false),
                )
        )
        .subcommand(
            Command::new("version")
                .about("Gets the installed version of the Minecraft server")
                .arg(
                    Arg::new("profile")
                        .long("profile")
                        .short('p')
                        .help("The profile to get the version of")
                        .num_args(1)
                        .required(false),
                )
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("update")
                .about("Updates the installed version of the Minecraft server")
                .arg(
                    Arg::new("profile")
                        .long("profile")
                        .short('p')
                        .help("The profile to update")
                        .num_args(1)
                        .required(false),
                )
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("profile")
                .about("Manages profiles")
                .subcommand(
                    Command::new("create")
                        .about("Creates a new profile")
                        .arg(
                            Arg::new("name")
                                .help("The name of the profile to create")
                                .num_args(1)
                                .required(true),
                        )
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Deletes a profile")
                        .arg(
                            Arg::new("name")
                                .help("The name of the profile to delete")
                                .num_args(1)
                                .required(false),
                        )
                        .arg_required_else_help(false),
                )
                .subcommand(
                    Command::new("use")
                        .about("Switches to a profile")
                        .arg(
                            Arg::new("name")
                                .help("The name of the profile to switch to")
                                .num_args(1)
                                .required(false),
                        )
                        .arg_required_else_help(false),
                )
                .subcommand(
                    Command::new("list")
                        .about("Lists all profiles")
                        .arg(
                            Arg::new("include_version")
                                .long("version")
                                .short('v')
                                .help("Include the version of each profile")
                                .num_args(0)
                                .default_value("false")
                                .value_parser(clap::value_parser!(bool))
                                .required(false),
                        )
                        .arg_required_else_help(false),
                )
                .arg_required_else_help(true)
                .subcommand_required(true),
        )
        .subcommand(
            Command::new("backup")
                .about("Manages backups")
                .subcommand(
                    Command::new("create")
                        .about("Creates a new backup")
                        .arg(
                            Arg::new("name")
                                .help("The name of the backup to create")
                                .num_args(1)
                                .required(true),
                        )
                        .arg(
                            Arg::new("format")
                                .help("The format of the backup to create (options: zip, tar, tar.gz, tar.xz)")
                                .num_args(1)
                                .default_value("zip")
                                .required(false),
                        )
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Deletes a backup")
                        .arg(
                            Arg::new("name")
                                .help("The name of the backup to delete")
                                .num_args(1)
                                .required(false),
                        )
                        .arg_required_else_help(false),
                )
                .subcommand(
                    Command::new("restore")
                        .about("Restores a backup")
                        .arg(
                            Arg::new("name")
                                .help("The name of the backup to restore")
                                .num_args(1)
                                .required(false),
                        )
                        .arg_required_else_help(false),
                )
                .subcommand(
                    Command::new("list")
                        .about("Lists all backups")
                        .arg_required_else_help(false),
                )
                .arg_required_else_help(true)
                .subcommand_required(true),
        )
        .subcommand(
            Command::new("mods")
                .about("Manages mods")
                .subcommand(
                    Command::new("list")
                        .about("Lists all mods")
                        .arg_required_else_help(false),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Deletes selected mods")
                        .arg_required_else_help(false),
                )
                .arg_required_else_help(true)
                .subcommand_required(true),
        )
        .subcommand(
            Command::new("java")
                .about("Manages Java versions")
                .subcommand(
                    Command::new("list")
                        .about("Lists all Java versions")
                        .arg_required_else_help(false),
                )
                .subcommand(
                    Command::new("use")
                        .about("Switches to a Java version")
                        .arg(
                            Arg::new("version")
                                .help("The version of Java to install")
                                .num_args(1)
                                .value_parser(clap::value_parser!(u8).range(8..=50))
                                .required(false),
                        )
                        .arg_required_else_help(false),
                )
                .subcommand(
                    Command::new("install")
                        .about("Installs a new Java version")
                        .arg(
                            Arg::new("version")
                                .help("The version of Java to install")
                                .num_args(1)
                                .value_parser(clap::value_parser!(u8).range(8..=50))
                                .required(false),
                        )
                        .arg_required_else_help(false),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Deletes a Java version")
                        .arg(
                            Arg::new("version")
                                .help("The version of Java to delete")
                                .num_args(1)
                                .value_parser(clap::value_parser!(u8).range(8..=50))
                                .required(false),
                        )
                        .arg_required_else_help(false),
                )
                .arg_required_else_help(true)
                .subcommand_required(true),
        )
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            std::process::exit(commands::init::init(sub_matches, None, None).await)
        }
        Some(("config", sub_matches)) => {
            std::process::exit(commands::config::config(sub_matches).await)
        }
        Some(("install", sub_matches)) => {
            std::process::exit(commands::install::install(sub_matches).await)
        }
        Some(("start", sub_matches)) => {
            std::process::exit(commands::start::start(sub_matches).await)
        }
        Some(("stop", sub_matches)) => std::process::exit(commands::stop::stop(sub_matches).await),
        Some(("attach", sub_matches)) => {
            std::process::exit(commands::attach::attach(sub_matches).await)
        }
        Some(("status", sub_matches)) => {
            std::process::exit(commands::status::status(sub_matches).await)
        }
        Some(("lookup", sub_matches)) => {
            std::process::exit(commands::lookup::lookup(sub_matches).await)
        }
        Some(("query", sub_matches)) => {
            std::process::exit(commands::query::query(sub_matches).await)
        }
        Some(("version", sub_matches)) => {
            std::process::exit(commands::version::version(sub_matches).await)
        }
        Some(("update", sub_matches)) => {
            std::process::exit(commands::update::update(sub_matches).await)
        }
        Some(("profile", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_matches)) => {
                std::process::exit(commands::profile::create::create(sub_matches).await)
            }
            Some(("delete", sub_matches)) => {
                std::process::exit(commands::profile::delete::delete(sub_matches).await)
            }
            Some(("use", sub_matches)) => {
                std::process::exit(commands::profile::r#use::r#use(sub_matches).await)
            }
            Some(("list", sub_matches)) => {
                std::process::exit(commands::profile::list::list(sub_matches).await)
            }
            _ => unreachable!(),
        },
        Some(("mods", sub_matches)) => match sub_matches.subcommand() {
            Some(("list", sub_matches)) => {
                std::process::exit(commands::mods::list::list(sub_matches).await)
            }
            Some(("delete", sub_matches)) => {
                std::process::exit(commands::mods::delete::delete(sub_matches).await)
            }
            _ => unreachable!(),
        },
        Some(("java", sub_matches)) => match sub_matches.subcommand() {
            Some(("list", sub_matches)) => {
                std::process::exit(commands::java::list::list(sub_matches).await)
            }
            Some(("use", sub_matches)) => {
                std::process::exit(commands::java::r#use::r#use(sub_matches).await)
            }
            Some(("install", sub_matches)) => {
                std::process::exit(commands::java::install::install(sub_matches).await)
            }
            Some(("delete", sub_matches)) => {
                std::process::exit(commands::java::delete::delete(sub_matches).await)
            }
            _ => unreachable!(),
        },
        _ => cli().print_help().unwrap(),
    }
}
