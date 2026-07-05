mod functions;
mod models;
mod settings;

use functions::{extract_bind::extract_bind, parse_args::parse_args, show_commands::show_commands};
use models::commands::Commands;
use models::config::Config;
use settings::constants::constants;

use std::fs::File;
use std::io::BufReader;

fn main() {
    let args = parse_args();

    let config_path = args
        .value_of(constants::ARG_CONFIG_PATH)
        .map(String::from)
        .unwrap_or_else(|| {
            constants::get_default_config_path()
                .expect("NA")
                .to_string_lossy()
                .into_owned()
        });

    let cfg: Config = Config::new(String::from(config_path));
    let path = cfg.get_path();

    if let Ok(f) = File::open(&path) {
        let reader = BufReader::new(f);
        let commands = Commands::from_reader(reader);
        show_commands(commands);
    } else {
        eprintln!("Error: file not found: `{}`", &path);
    }
}
