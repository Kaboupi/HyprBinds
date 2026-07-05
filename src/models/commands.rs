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
        let mut commands: Vec<HashMap<&'static str, String>> = Vec::new();
        let mut current_block: Option<String> = None;

        for line_result in reader.lines() {
            let line = match line_result {
                Ok(l) => l,
                Err(_) => break,
            };

            let trimmed = line.trim();

            if trimmed.contains("hl.bind") {
                if let Some(block) = current_block {
                    if let Some(parsed) = extract_bind(&block) {
                        commands.push(parsed);
                    }
                }
                current_block = Some(line);
            } else if let Some(ref mut block) = current_block {
                block.push('\n');
                block.push_str(&line);
            }
        }

        if let Some(block) = current_block {
            if let Some(parsed) = extract_bind(&block) {
                commands.push(parsed);
            }
        }

        Commands::new(commands)
    }
}
