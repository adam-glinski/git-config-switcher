// mod types;

use clap::{arg, command, ArgMatches, Command};
use std::collections::HashMap;


/// @TODO: Change name to smthign more accurate
pub fn show_cli() -> ArgMatches {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Adds new author to the list.")
                .args([
                    arg!([NAME] "Author name."),
                    arg!([EMAIL] "Author email."),
                    arg!([ALIAS] "Config alias."),
                ]),
        )
        .subcommand(Command::new("list").about("List all saved configs."))
        .subcommand(
            Command::new("remove")
                .about("Remove config.")
                .arg(arg!([ALIAS] "Config alias.")),
        )
        .subcommand(
            Command::new("switch")
                .about("Switch config.")
                .arg(arg!([ALIAS] "Config alias.")),
        )
        .get_matches();

    return matches;
}

pub fn resolve_params(matches: &ArgMatches, function_map: HashMap<String, fn(&ArgMatches)>) {
    let on_add = function_map.get("add").unwrap();
    match matches.subcommand() {
        Some(("add", sub_matches)) => on_add(sub_matches),

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
