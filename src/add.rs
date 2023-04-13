use crate::types;
use crate::utils;

use clap::ArgMatches;

// Json
use serde_json;

// File 
// use std::fs::File;
// use std::env;
use std::{dbg};

fn extract_config(matches: &ArgMatches) -> (String, types::Config) {
    let Some(ref name) = matches.get_one::<String>("name") else { panic!("Missing name! Use help command to view usage.") };
    let Some(ref email) = matches.get_one::<String>("email") else { panic!("Missing email! Use help command to view usage.") };
    let Some(ref alias) = matches.get_one::<String>("alias") else { panic!("Missing email! Use help command to view usage.") };


    (alias.to_string(), types::Config{name: name.to_string(), email: email.to_string()})
}


pub fn on_add(sub_matches: &ArgMatches) -> serde_json::Result<()> {
    let mut _path:String = utils::get_config_dir();
    _path.push_str(r"/data.json");

    let mut data: serde_json::Value = serde_json::Value::Null;

    let (alias, config) = extract_config(sub_matches);

    let seraizalized_config = serde_json::to_string(&config).unwrap();
    data[alias] = serde_json::json!(seraizalized_config);

    let _deserialized_config: types::Config = serde_json::from_str(&seraizalized_config).unwrap();

    // serde_json::to_writer(writer, value)
    // let output = &File::create(&path).unwrap();
    // serde_json::to_writer(output, &seraizalized_config);
    dbg!(&data);
    Ok(())
}


// #[cfg(tests)]
// mod tests{
//     #[test]
// }