use clap::{command, Command, arg, ArgMatches};

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
                    arg!([ALIAS] "Config alias.")
                    ])
        ).arg_required_else_help(true)
        .get_matches();

    return matches;
}