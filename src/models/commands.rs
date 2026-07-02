use std::collections::HashMap;

#[derive(Debug)]
pub struct Commands {
    pub commands: Vec<HashMap<&'static str, String>>,
}

impl Commands {
    pub fn new() -> Commands {
        Commands {
            commands: Vec::new(),
        }
    }

    pub fn from(items: Vec<HashMap<&'static str, String>>) -> Commands {
        Commands { commands: items }
    }

    pub fn show(&self) -> () {
        for (idx, item) in self.commands.iter().enumerate() {
            let kb = item.get("kb").unwrap();
            let desc = item.get("desc").unwrap();

            println!("{}: kb = \"{}\", desc = \"{}\"", idx, kb, desc);
        }
    }
}
