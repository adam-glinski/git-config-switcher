use crate::types;
use crate::utils;

use clap::ArgMatches;

// Json
use serde_json;

// File 
use std::fs::File;
// use std::env;



pub fn on_add(sub_matches: &ArgMatches) {
    let mut path = utils::get_config_dir().push_str(r"/data.json");

    // let name = sub_matches.get_one::<String>("NAME").unwrap().to_string();
    // let email = sub_matches.get_one::<String>("EMAIL").unwrap().to_string();
    // let alias = sub_matches.get_one::<String>("ALIAS").unwrap().to_string();


    let Some(ref name) = sub_matches.get_one::<String>("name") else { panic!("Missing name! Use help command to view usage.") };
    let Some(ref email) = sub_matches.get_one::<String>("email") else { panic!("Missing email! Use help command to view usage.") };
    let Some(ref alias) = sub_matches.get_one::<String>("alias") else { panic!("Missing email! Use help command to view usage.") };

    let config = types::Config{name: name.to_string(), email: email.to_string(), alias: alias.to_string()};
    let seraizalized_config = serde_json::to_string(&config).unwrap();

    let deserialized_config: types::Config = serde_json::from_str(&seraizalized_config).unwrap();

    // serde_json::to_writer(writer, value)
    let output = &File::create(&path).unwrap();
    serde_json::to_writer(output, &seraizalized_config);
    dbg!(&deserialized_config);
}