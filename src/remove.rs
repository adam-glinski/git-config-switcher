use clap::ArgMatches;
use colored::Colorize;

use crate::types::ConfigsMap;
use crate::utils::save_configs_to_file;

pub fn on_remove(matches: &ArgMatches, configs_map: &mut ConfigsMap) {
    let Some(ref alias) = matches.get_one::<String>("alias")
        else { panic!("Failed to read alias from ArgMatches") };
    configs_map.remove(&alias.to_string());

    save_configs_to_file(&configs_map).expect("Failed to save config.");
    println!("Successfully removed {} config!", alias.italic().bold());
}
