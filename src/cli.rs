// mod types;

use crate::types;
use clap::{ArgMatches, Command, Arg};
use std::collections::HashMap;


/// @TODO: Change name to smthign more accurate
pub fn show_cli() -> ArgMatches {
    let matches = Command::new("gitswitcher")
        .version("0.1.0")
        .about("Git config manager.")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Adds new author to the list.")
                .arg(Arg::new("name").help("Author name.").required(true))
                .arg(Arg::new("email").help("Author email.").required(true))
                .arg(Arg::new("alias").help("Config alias.").required(true))
                .visible_alias("a")
        )
        .subcommand(
            Command::new("list").about("List all saved configs.")
            .visible_alias("l")
            .visible_alias("ls")
        )
        .subcommand(
            Command::new("remove")
                .about("Remove config.")
                .arg(Arg::new("alias").help("Config alias meant to be removed.").required(true))
                .visible_alias("r")
        )
        .subcommand(
            Command::new("switch")
                .about("Switch config.")
                .arg(Arg::new("alias").help("Config alias that you want to switch to.").required(true))
                .visible_alias("s")
        )
        .get_matches();


    return matches;
}

pub fn resolve_params(matches: &ArgMatches, function_map: HashMap<String, types::Callback>, configs_map: &HashMap<String, types::Config>) -> Result<(), serde_json::Error> {
    let on_add = function_map.get("add").unwrap();
    match matches.subcommand() {
        // @TODO: rearrange
        Some(("add", sub_matches)) => on_add(sub_matches, configs_map),

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
