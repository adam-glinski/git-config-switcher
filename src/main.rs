mod add;
mod cli;
mod types;
mod utils;

use std::{collections::HashMap};



fn main() {
    let matches = cli::show_cli();
    let mut functions: HashMap<String, types::Callback> = HashMap::new();
    functions.insert("add".to_string(), add::on_add);

    cli::resolve_params(&matches, functions);
}
