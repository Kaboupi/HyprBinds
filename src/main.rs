mod models;
mod settings;
mod utilities;

use models::commands::Commands;
use models::config::Config;
use settings::constants::constants;
use utilities::functions::{extract_bind, parse_args};

use std::fs::File;
use std::io::BufReader;

fn main() {
    let args = parse_args();
    let config_path = args
        .value_of(constants::ARG_CONFIG_PATH)
        .unwrap_or_else(|| constants::DEFAULT_CONFIG_PATH);

    let cfg: Config = Config::new(String::from(config_path));
    let path = cfg.get_path();

    if let Ok(f) = File::open(&path) {
        let reader = BufReader::new(f);
        let commands = Commands::from_reader(reader);
        commands.show(); // commands - Vec<HashMap<&str, String>>
    } else {
        eprintln!("Error: file not found: `{}`", &path);
    }
}
