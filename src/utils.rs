use crate::types;
use std::collections::HashMap;
use std::env;
use std::fs::{File, create_dir_all};
use serde_json::{to_writer_pretty, from_reader};
use std::dbg;
use std::io::BufReader;


fn get_config_dir() -> String {
    let mut _file_path = String::new();
    if cfg!(windows) {
        _file_path = env::var("APPDATA").unwrap();
        _file_path.push_str("\\.git_config_switcher\\")
    } else if cfg!(unix) {
        _file_path = String::from("~/.git_config_switcher/");
    } else {
        panic!("Unsupported os!");
    }

    create_dir_all(&_file_path).expect("Failed to create directory!");
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
    let file = File::open(get_config_dir()).expect("Failed to open file");
    // dbg!(&file);
    let reader = BufReader::new(file);
    let map: Result<HashMap<String, types::Config>, serde_json::Error> = from_reader(reader);
    match map {
        Ok(map) => map,
        Err(_e) => {
            HashMap::new()
        }
    }
}