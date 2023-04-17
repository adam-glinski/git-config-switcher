mod add;
mod cli;
mod types;
mod utils;
mod list;

use std::collections::HashMap;


fn main() {
    let matches = cli::show_cli();
    let mut functions: HashMap<String, types::Callback> = HashMap::new();
    let mut configs_map = utils::load_configs_from_file();
    functions.insert("add".to_string(), add::on_add);


    if let Err(err) = cli::resolve_params(&matches, functions, &mut configs_map) {
        println!("Error: {}", err);
    }
}
