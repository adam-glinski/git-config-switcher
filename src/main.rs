mod add;
mod cli;
mod types;
mod utils;
mod list;
mod remove;
mod switch;


fn main() -> std::io::Result<()> {
    let matches = cli::show_cli();
    let mut configs_map = utils::load_configs_from_file();

    cli::resolve_params(&matches, &mut configs_map);
    Ok(())
}
