pub mod constants {
    use std::path::PathBuf;

    pub const APP_NAME: &str = "hyprbinds";
    pub const APP_VERSION: &str = "v0.0.2";

    pub const ARG_CONFIG_PATH: &str = "config-path";

    const RELATIVE_CONFIG_PATH: &str = ".config/hypr/keybinds.lua";
    pub fn get_default_config_path() -> Option<PathBuf> {
        home::home_dir().map(|home| home.join(RELATIVE_CONFIG_PATH))
    }

    pub const RE_PATTERN: &str =
        r#"(?s)hl\.bind\((?<kb>[^,)]+)(?:.*description.*"(?<desc>[^"]*)")?"#;

    pub const K_KB: &str = "kb";
    pub const K_DESC: &str = "desc";

    pub const DESC_EMPTY: &str = "No description provided.";
}
