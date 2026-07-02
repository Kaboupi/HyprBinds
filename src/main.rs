#![allow(unused_imports, dead_code, unused_variables)]

mod settings;
use settings::constants::constants;

mod models;
use models::commands::Commands;
use models::config::Config;

use clap::{App, Arg};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn parse_args() -> clap::ArgMatches<'static> {
    App::new(constants::APP_NAME)
        .version(constants::APP_VERSION)
        .about("prints out keybinds for hyprland")
        .arg(
            Arg::with_name(constants::ARG_CONFIG_PATH)
                .help("The file to search in")
                .takes_value(true)
                .required(false),
        )
        .get_matches()
}

fn parse_commands<T: BufRead + Sized>(reader: T, re: Regex) -> Commands {
    let mut tmp_commands = Vec::new();
    let re_cleanup: Regex = Regex::new(&constants::RE_PATTERN_CLEANUP).unwrap();

    for (idx, line_) in reader.lines().enumerate() {
        if let Ok(line) = line_ {
            match re.captures(&line) {
                Some(caps) => {
                    // TODO: Add description
                    let kb = (
                        "kb",
                        String::from(re_cleanup.replace_all(&caps["cmd"], " + $1")),
                    );
                    let desc = ("desc", String::from("none"));
                    let map = [kb, desc];
                    tmp_commands.push(HashMap::from(map));
                }
                None => (),
            }
        }
    }

    Commands::from(tmp_commands)
}

fn main() {
    let args = parse_args();
    let config_path = args
        .value_of(constants::ARG_CONFIG_PATH)
        .unwrap_or_else(|| constants::DEFAULT_CONFIG_PATH);

    let cfg: Config = Config::new(String::from(config_path));
    let path = cfg.get_path();

    if let Ok(f) = File::open(&path) {
        let reader = BufReader::new(f);
        let re: Regex = Regex::new(&constants::RE_PATTERN_EXTRACT).unwrap();
        let commands = parse_commands(reader, re);

        commands.show();
    } else {
        eprintln!("Error: file not found: `{}`", &path);
    }
}
