use crate::java;

use clap::ArgMatches;
use colored::Colorize;
use dialoguer::{FuzzySelect, theme::ColorfulTheme};

pub async fn delete(matches: &ArgMatches) -> i32 {
    let version = matches.get_one::<u8>("version");

    println!("{}", "listing java versions...".bright_black());

    let list = java::installed();

    println!(
        "{} {}",
        "listing java versions...".bright_black(),
        "DONE".green().bold()
    );
    println!();

    let version = if let Some(version) = version {
        if !list.iter().any(|(v, _)| v == version) {
            println!(
                "{} {} {}",
                "java".bright_black(),
                version.to_string().cyan(),
                "not installed".red()
            );
            return 1;
        }

        *version
    } else {
        if list.is_empty() {
            println!("{}", "no java versions to delete".red());
            return 1;
        }

        let version = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select java version to delete")
            .items(
                &list
                    .iter()
                    .map(|(version, _)| format!("java {}", version))
                    .collect::<Vec<String>>(),
            )
            .default(0)
            .max_length(5)
            .interact()
            .unwrap();
        println!();

        list[version].0
    };

    println!(
        "{} {} {}",
        "removing java".bright_black(),
        version.to_string().cyan(),
        "...".bright_black()
    );

    java::remove(version);

    println!(
        "{} {} {} {}",
        "removing java".bright_black(),
        version.to_string().cyan(),
        "...".bright_black(),
        "DONE".green().bold()
    );

    0
}
