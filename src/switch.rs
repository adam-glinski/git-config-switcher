use clap::ArgMatches;
use std::process::Command;
use crate::types::ConfigsMap;

pub fn on_switch(matches: &ArgMatches, configs_map: &mut ConfigsMap) {
    let Some(ref alias) = matches.get_one::<String>("alias")
        else { panic!("Failed to read alias from ArgMatches") };

    // TODO: Make sure the alias exists in the map
    let config = configs_map.get(&alias.to_string()).unwrap();
    let git_name_output = Command::new("git").args(["config", "--global", "user.name", &config.name]).output().unwrap();
    let git_email_output = Command::new("git").args(["config", "--global", "user.email", &config.email]).output().unwrap();
    // println!("{}, {}", git_name_output.stdout, git_email_output.stdout);
}