mod cli;

fn main() {
    let matches = cli::show_cli();

    match matches.subcommand() {
        Some(("add", sub_matches)) => println!(
            "'add' was used, name is: {:?}, email is: {:?}, alias is: {:?}",
            sub_matches.get_one::<String>("NAME"),
            sub_matches.get_one::<String>("EMAIL"),
            sub_matches.get_one::<String>("ALIAS")
        ),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}