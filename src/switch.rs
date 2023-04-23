use crate::types::ConfigsMap;
use clap::ArgMatches;
use std::io::{stdout, Write};
use std::process::Command;

pub fn on_switch(matches: &ArgMatches, configs_map: &mut ConfigsMap) {
    let Some(ref alias) = matches.get_one::<String>("alias")
        else { panic!("Failed to read alias from ArgMatches") };

    match configs_map.get(&alias.to_string()) {
        Some(config) => {
            let git_name_output = Command::new("git")
                .args(["config", "--global", "user.name", &config.name])
                .output()
                .unwrap();
            let git_email_output = Command::new("git")
                .args(["config", "--global", "user.email", &config.email])
                .output()
                .unwrap();

            stdout()
                .write_all(&git_name_output.stdout)
                .expect("Failed to print output from git_name_output");
            stdout()
                .write_all(&git_email_output.stdout)
                .expect("Failed to print output from git_email_output");
        }

        None => {
            println!("Invalid alias!");
        }
    }
}
