use crate::types;
use serde_json::{from_reader, to_writer_pretty};
use std::collections::HashMap;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::BufReader;
use std::io::ErrorKind;

fn get_config_dir() -> String {
    let mut _file_path = String::new();
    if cfg!(windows) {
        _file_path = env::var("APPDATA").unwrap();
        _file_path.push_str("\\.git_config_switcher\\")
    } else if cfg!(unix) {
        // _file_path = String::from("~/.git_config_switcher/");
        _file_path = env::var("HOME").unwrap();
        _file_path.push_str("/.git_config_switcher/");
    } else {
        panic!("Unsupported os!");
    }

    create_dir_all(&_file_path).unwrap_or_else(|e| panic!("Error while creating directory: {e}"));
    _file_path.push_str("data.json");
    return _file_path;
}

pub fn save_configs_to_file(map: &HashMap<String, types::Config>) -> std::io::Result<()> {
    let file = File::create(get_config_dir())?;
    to_writer_pretty(file, map)?;
    Ok(())
}

// Perhaps should return Result<*> to keep the style referring to save_configs_to_file
pub fn load_configs_from_file() -> HashMap<String, types::Config> {
    let path = get_config_dir();
    // TODO: Reformat to look better !!!
    let mut config_file = File::open(&path);
    if let Err(e) = &config_file {
        if e.kind() == ErrorKind::NotFound {
            config_file = File::create(&path);
        }
    }
    // dbg!(&file);
    let reader = BufReader::new(config_file.unwrap());
    let map: Result<HashMap<String, types::Config>, serde_json::Error> = from_reader(reader);
    match map {
        Ok(map) => map,
        Err(_e) => HashMap::new(),
    }
}
