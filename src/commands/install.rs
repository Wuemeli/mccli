use crate::{api, config, detached, jar, modpack};

use clap::ArgMatches;
use colored::Colorize;
use dialoguer::{FuzzySelect, Input, Select, theme::ColorfulTheme};

pub async fn install(matches: &ArgMatches) -> i32 {
    let mut config = config::Config::new(".mcvcli.json", false);
    let wipe = matches.get_one::<bool>("wipe").expect("required");

    if detached::status(config.pid) {
        println!(
            "{} {}",
            "server is currently running, use".red(),
            "mcvcli stop".cyan()
        );
        return 1;
    }

    let server_jarfile = if let Some(file) = matches.get_one::<String>("file") {
        match file.as_str() {
            "install" => 0,
            "modrinth" => 1,
            _ => {
                println!("{} {}", file.cyan(), "not found".red());
                return 1;
            }
        }
    } else {
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Server Jar File")
            .default(0)
            .item("Install New (Jar)")
            .item("Install New (Modrinth Modpack)")
            .interact()
            .unwrap()
    };

    match server_jarfile {
        0 => {
            if *wipe {
                println!("{}", "Wiping server directory...".bright_black());

                let entries = std::fs::read_dir(".").unwrap();
                for entry in entries {
                    let entry = entry.unwrap();
                    let path = entry.path();

                    if path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                        .starts_with(".mcvcli")
                    {
                        continue;
                    }

                    if path.is_dir() {
                        std::fs::remove_dir_all(&path).unwrap();
                    } else {
                        std::fs::remove_file(&path).unwrap();
                    }
                }

                println!(
                    "{} {}",
                    "Wiping server directory...".bright_black(),
                    "DONE".green().bold()
                );
            }

            let java = if let Some(Ok(build_id)) =
                matches.get_one::<String>("build").map(|b| b.parse::<u32>())
            {
                println!(
                    "{} {}",
                    "getting server build...".bright_black(),
                    "...".bright_black()
                );

                if let Ok((server_build, versions)) = api::mcjars::lookup_id(build_id).await {
                    println!(
                        "{} {}",
                        "getting server build...".bright_black(),
                        "DONE".green().bold()
                    );

                    let server_version = server_build
                        .version_id
                        .as_ref()
                        .unwrap_or_else(|| server_build.project_version_id.as_ref().unwrap());

                    println!(
                        "{} {} {} {}",
                        "installing".bright_black(),
                        server_version.cyan(),
                        server_build.name.cyan(),
                        "...".bright_black()
                    );

                    jar::install(&server_build, ".", 1).await.unwrap();

                    println!(
                        "{} {} {} {} {}",
                        "installing".bright_black(),
                        server_version.cyan(),
                        server_build.name.cyan(),
                        "...".bright_black(),
                        "DONE".green().bold()
                    );

                    versions
                        .get(server_version)
                        .unwrap_or(versions.last().unwrap().1)
                        .java
                } else {
                    println!(
                        "{} {} {}",
                        "server build".red(),
                        build_id.to_string().cyan(),
                        "not found!".red()
                    );
                    return 1;
                }
            } else {
                println!("{}", "getting server types...".bright_black());

                let types = api::mcjars::types().await.unwrap();

                println!(
                    "{} {}",
                    "getting server types...".bright_black(),
                    "DONE".green().bold()
                );

                let server_type = if let Some(r#type) = matches.get_one::<String>("type") {
                    if !types.contains_key(&r#type.to_uppercase()) {
                        println!(
                            "{} {} {}",
                            "server type".red(),
                            r#type.to_string().cyan(),
                            "not found!".red()
                        );
                        return 1;
                    }

                    &r#type.to_uppercase()
                } else {
                    let server_type = FuzzySelect::with_theme(&ColorfulTheme::default())
                        .with_prompt("Server Jar File")
                        .default(0)
                        .items(&types.values().map(|t| &t.name).collect::<Vec<&String>>())
                        .max_length(10)
                        .interact()
                        .unwrap();

                    types.keys().nth(server_type).unwrap()
                };

                println!(
                    "{} {} {}",
                    "getting server versions for".bright_black(),
                    types.get(server_type).unwrap().name.to_string().cyan(),
                    "...".bright_black()
                );

                let versions = api::mcjars::versions(server_type).await.unwrap();

                println!(
                    "{} {} {} {}",
                    "getting server versions for".bright_black(),
                    types.get(server_type).unwrap().name.to_string().cyan(),
                    "...".bright_black(),
                    "DONE".green().bold()
                );

                let server_version = if let Some(version) = matches.get_one::<String>("version") {
                    if !versions.contains_key(version) {
                        println!(
                            "{} {} {}",
                            "server version".red(),
                            version.to_string().cyan(),
                            "not found!".red()
                        );
                        return 1;
                    }

                    version
                } else {
                    let server_version = FuzzySelect::with_theme(&ColorfulTheme::default())
                        .with_prompt("Jar Version")
                        .default(0)
                        .items(&versions.keys().rev().collect::<Vec<&String>>())
                        .max_length(10)
                        .interact()
                        .unwrap();

                    versions.keys().rev().nth(server_version).unwrap()
                };

                println!(
                    "{} {} {}",
                    "getting server builds for".bright_black(),
                    server_version.to_string().cyan(),
                    "...".bright_black()
                );

                let builds = api::mcjars::builds(server_type, server_version)
                    .await
                    .unwrap();

                println!(
                    "{} {} {} {}",
                    "getting server builds for".bright_black(),
                    server_version.to_string().cyan(),
                    "...".bright_black(),
                    "DONE".green().bold()
                );

                let server_build = if let Some(build) = matches.get_one::<String>("build") {
                    if build.as_str() == "latest" {
                        builds.first().unwrap()
                    } else if let Some(build) = builds.iter().find(|b| &b.name == build) {
                        build
                    } else {
                        println!(
                            "{} {} {}",
                            "server build".red(),
                            build.to_string().cyan(),
                            "not found!".red()
                        );
                        return 1;
                    }
                } else {
                    let server_build = FuzzySelect::with_theme(&ColorfulTheme::default())
                        .with_prompt("Jar Build")
                        .default(0)
                        .items(&builds.iter().map(|b| &b.name).collect::<Vec<&String>>())
                        .max_length(10)
                        .interact()
                        .unwrap();

                    &builds[server_build]
                };

                println!(
                    "{} {} {} {}",
                    "installing".bright_black(),
                    server_version.cyan(),
                    server_build.name.cyan(),
                    "...".bright_black()
                );

                jar::install(server_build, ".", 1).await.unwrap();

                println!(
                    "{} {} {} {} {}",
                    "installing".bright_black(),
                    server_version.cyan(),
                    server_build.name.cyan(),
                    "...".bright_black(),
                    "DONE".green().bold()
                );

                versions.get(server_version).unwrap().java
            };

            config.modpack_slug = None;
            config.modpack_version = None;
            config.java_version = java;
            config.save();
        }
        1 => {
            let mut projects = api::modrinth::projects(
                "",
                "[[\"project_type:modpack\"],[\"server_side != unsupported\"]]",
            )
            .await
            .unwrap();
            let mut project;

            loop {
                let modpack = FuzzySelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Modpack?")
                    .default(0)
                    .item("Search")
                    .items(
                        &projects
                            .iter()
                            .map(|p| {
                                format!(
                                    "{:17} {}",
                                    format!(
                                        "{} - {}",
                                        p.versions.first().unwrap(),
                                        p.versions.last().unwrap()
                                    ),
                                    p.title
                                )
                            })
                            .collect::<Vec<String>>(),
                    )
                    .max_length(10)
                    .interact()
                    .unwrap();

                project = modpack;

                if modpack == 0 {
                    let search = Input::<String>::new()
                        .with_prompt("Search")
                        .interact()
                        .unwrap();

                    projects = api::modrinth::projects(
                        &search,
                        "[[\"project_type:modpack\"],[\"server_side != unsupported\"]]",
                    )
                    .await
                    .unwrap();
                } else {
                    break;
                }
            }

            let project = &projects[project - 1];

            println!();
            println!(
                "{} {} {}",
                "getting versions for".bright_black(),
                project.title.cyan(),
                "...".bright_black()
            );

            let versions = api::modrinth::versions(project.project_id.as_ref().unwrap())
                .await
                .unwrap();
            let versions = versions
                .iter()
                .filter(|v| !v.files.is_empty())
                .filter(|v| v.name.is_some() || v.version_number.is_some())
                .collect::<Vec<&api::modrinth::Version>>();

            println!(
                "{} {} {} {}",
                "getting versions for".bright_black(),
                project.title.cyan(),
                "...".bright_black(),
                "DONE".green().bold()
            );

            println!();
            println!("{}", project.title.cyan().bold());
            println!(
                "  {} {}",
                "description:".bright_black(),
                project.description.cyan()
            );
            println!(
                "  {} {}",
                "project id: ".bright_black(),
                project.project_id.as_ref().unwrap().cyan()
            );
            println!(
                "  {} {}",
                "downloads:  ".bright_black(),
                project.downloads.to_string().cyan()
            );
            println!();

            let modpack_version = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Modpack Version?")
                .default(0)
                .items(
                    &versions
                        .iter()
                        .map(|v| {
                            format!(
                                "{:8} {}",
                                v.game_versions.first().unwrap(),
                                v.name
                                    .as_ref()
                                    .unwrap_or(v.version_number.as_ref().unwrap())
                            )
                        })
                        .collect::<Vec<String>>(),
                )
                .max_length(5)
                .interact()
                .unwrap();

            let modpack_version = &versions[modpack_version];

            println!();

            if *wipe {
                println!("{}", "Wiping server directory...".bright_black());

                let entries = std::fs::read_dir(".").unwrap();
                for entry in entries {
                    let entry = entry.unwrap();
                    let path = entry.path();

                    if path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                        .starts_with(".mcvcli")
                    {
                        continue;
                    }

                    if path.is_dir() {
                        std::fs::remove_dir_all(&path).unwrap();
                    } else {
                        std::fs::remove_file(&path).unwrap();
                    }
                }

                println!(
                    "{} {}",
                    "Wiping server directory...".bright_black(),
                    "DONE".green().bold()
                );
            }

            println!(
                "{} {} {}",
                "installing".bright_black(),
                project.title.cyan(),
                "...".bright_black()
            );

            modpack::install(".", modpack_version).await;

            config.jar_file = "server.jar".to_string();
            config.modpack_slug = Some(project.project_id.clone().unwrap());
            config.modpack_version = Some(modpack_version.id.clone());
            let detected = jar::detect(".", &config).await;

            if let Some(([build, _], versions, _)) = detected {
                config.java_version = versions
                    .get(
                        &build
                            .version_id
                            .unwrap_or(build.project_version_id.unwrap_or("unknown".to_string())),
                    )
                    .unwrap_or(versions.last().unwrap().1)
                    .java;
            }

            config.save();

            println!(
                "{} {} {} {}",
                "installing".bright_black(),
                project.title.cyan(),
                "...".bright_black(),
                "DONE".green().bold()
            );
        }
        _ => unreachable!(),
    }

    0
}
