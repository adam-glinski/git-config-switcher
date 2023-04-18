use crate::types::{Config, ConfigsMap};
use crate::utils;

use clap::ArgMatches;
use std::dbg;

fn extract_config(matches: &ArgMatches) -> (String, Config) {
    let Some(ref name) = matches.get_one::<String>("name") else { panic!("Missing name! Use help command to view usage.") };
    let Some(ref email) = matches.get_one::<String>("email") else { panic!("Missing email! Use help command to view usage.") };
    let Some(ref alias) = matches.get_one::<String>("alias") else { panic!("Missing alias! Use help command to view usage.") };

    (
        alias.to_string(),
        Config {
            name: name.to_string(),
            email: email.to_string(),
        },
    )
}

pub fn on_add(
    sub_matches: &ArgMatches,
    configs_map: &mut ConfigsMap,
) {

    let (alias, config) = extract_config(sub_matches);
    configs_map.insert(alias, config);

    utils::save_configs_to_file(configs_map).expect("Failed to save config!");
}