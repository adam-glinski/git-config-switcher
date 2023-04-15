use crate::types;
use std::collections::HashMap;
use std::env;
use std::fs;

pub fn get_config_dir() -> String {
    let mut _path = String::new();
    if cfg!(windows) {
        _path = env::var("APPDATA").unwrap();
        _path.push_str(r"\.git_config_switcher")
    } else if cfg!(unix) {
        _path = String::from("~/.git_config_switcher");
    } else {
        panic!("Unsupported os!");
    }

    return _path;
}

pub fn get_configs_map() -> HashMap<String, types::Config> {
    let file_path = get_config_dir().push_str("/data.json");
    let file_raw = fs::read_to_string(&file_path);

}
