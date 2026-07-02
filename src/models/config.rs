use std::path::Path;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    config_path: String,
    config_type: String,
}

impl Config {
    pub fn new(config_path: String) -> Self {
        let extension = Self::get_extension(&config_path);

        Config {
            config_path: config_path,
            config_type: extension,
        }
    }

    fn get_extension(path: &str) -> String {
        Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }

    pub fn get_path(self: Config) -> String {
        self.config_path
    }
}
