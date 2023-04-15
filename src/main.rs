mod add;
mod cli;
mod types;
mod utils;

use std::collections::HashMap;


fn main() {
    let matches = cli::show_cli();
    let mut functions: HashMap<String, types::Callback> = HashMap::new();
    functions.insert("add".to_string(), add::on_add);

    let mut configs_map: HashMap<String, types::Config> = HashMap::new();

    if let Err(err) = cli::resolve_params(&matches, functions, &configs_map) {
        println!("Error: {}", err);
    }
}
