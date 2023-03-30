// mod types;

use clap::{arg, ArgMatches, Command, Arg};
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
                .arg(Arg::new("name"))
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
    /*
    if let Some(ref matches) = matches.subcommand_matches("add") {
        if let Some(name) = matches.get_one::<String>("name") {
            println!("name {}", name.as_str());
        };
    }
     */
}

pub fn resolve_params(matches: &ArgMatches, function_map: HashMap<String, fn(&ArgMatches)>) {
    let on_add = function_map.get("add").unwrap();
    match matches.subcommand() {
        // @TODO: rearrange
        Some(("add", _sub_matches)) => on_add(matches),

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
