use crate::types;
use crate::utils;

use clap::ArgMatches;

// Json
use serde_json;

// File 
use std::fs::File;
use std::env;



pub fn on_add(sub_matches: &ArgMatches) {
    let mut _path = utils::get_config_dir();

    // let name = sub_matches.get_one::<String>("NAME").unwrap().to_string();
    // let email = sub_matches.get_one::<String>("EMAIL").unwrap().to_string();
    // let alias = sub_matches.get_one::<String>("ALIAS").unwrap().to_string();

    let Some(ref name) = sub_matches.get_one::<String>("name") else { panic!("Missing name! Use help command to view usage.") };
    let Some(ref email) = sub_matches.get_one::<String>("email") else { panic!("Missing email! Use help command to view usage.") };
    let Some(ref alias) = sub_matches.get_one::<String>("email") else { panic!("Missing email! Use help command to view usage.") };

    println!("Hello, {} your email is {}, alias {}", name, email, alias);
    println!("[*] Path: {}", _path);

    let config = types::Config{name: name.to_string(), email: email.to_string(), alias: alias.to_string()};
    let seraizalized_config = serde_json::to_string(&config);
    println!("{}", seraizalized_config.unwrap());   
}