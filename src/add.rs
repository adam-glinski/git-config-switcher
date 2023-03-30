use clap::ArgMatches;

// TODO: Rename param
pub fn on_add(sub_matches: &ArgMatches) {
//    if let Some(ref m) = sub_matches.subcommand_matches("add"){
//        if let Some(ref name) = m.get_one::<String>("name") {
//            println!("{}", name.as_str());
//        }
//    };

    let Some(ref m) = sub_matches.subcommand_matches("add") else { return };
    let Some(ref name) = m.get_one::<String>("name") else { panic!("Missing name! Use help command to view usage.") };
    let Some(ref mail) = m.get_one::<String>("email") else { panic!("Missing email! Use help command to view usage.") };

    println!("Hello, {} your email is {}", name, mail);
}