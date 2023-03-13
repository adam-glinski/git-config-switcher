mod add;
mod cli;

use std::collections::HashMap;

fn main() {
    let matches = cli::show_cli()Q;
    cli::resolve_params(matches, HashMap::from([("add", &(add::on_add))]));

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
