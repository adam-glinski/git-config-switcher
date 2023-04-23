mod add;
mod cli;
mod list;
mod remove;
mod switch;
mod types;
mod utils;

fn main() -> std::io::Result<()> {
    let matches = cli::show_cli();
    let mut configs_map = utils::load_configs_from_file();

    cli::resolve_params(&matches, &mut configs_map);
    Ok(())
}
