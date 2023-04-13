use std::env;

pub fn get_config_dir() -> String {
    let mut _path = String::new();
    if cfg!(windows) {
        _path = env::var("APPDATA").unwrap();
        _path.push_str(r"\.git_config_switcher")
    } else if cfg!(unix) {
        _path = String::from("~./.git_config_switcher");
    } else{
        panic!("Unsupported os!");
    }

    return _path;
}