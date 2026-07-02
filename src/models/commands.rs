use crate::extract_bind;
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug)]
pub struct Commands {
    pub commands: Vec<HashMap<&'static str, String>>,
}

impl Commands {
    pub fn new(items: Vec<HashMap<&'static str, String>>) -> Commands {
        Commands { commands: items }
    }

    pub fn from_reader<T: BufRead + Sized>(reader: T) -> Commands {
        let commands: Vec<HashMap<&'static str, String>> = reader
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| extract_bind(&line))
            .collect();

        Commands::new(commands)
    }

    pub fn show(&self) -> () {
        for (idx, item) in self.commands.iter().enumerate() {
            let kb = item.get("kb").unwrap();
            let desc = item.get("desc").unwrap();

            println!("{}: kb = \"{}\", desc = \"{}\"", idx, kb, desc);
        }
    }
}
