use std::env;

pub fn get_config_dir() -> String {
    let mut path = String::new();
    if cfg!(windows) {
        path = env::var("APPDATA").unwrap();
        path.push_str(r"\.git_config_switcher")
    } else if cfg!(unix) {
        path = String::from("~./.git_config_switcher");
    } else{
        panic!("Unsupported os!");
    }

    return path;
}