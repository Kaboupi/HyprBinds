pub mod constants {
    pub const APP_NAME: &str = "hyprbinds";
    pub const APP_VERSION: &str = "v0.0.1";

    pub const RE_PATTERN: &str =
        r#"hl\.bind\((?<raw_cmd>[^,)]+)(?:.*description.*"(?<desc>[^"]*)")?"#;

    pub const NO_DESCRIPTION: &str = "No description provided.";

    pub const ARG_CONFIG_PATH: &str = "config-path";
    // TODO: Replace absolute path
    pub const DEFAULT_CONFIG_PATH: &str = "/home/kaboupi/.config/hypr/keybinds.lua";
}
