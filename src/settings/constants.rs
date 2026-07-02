pub mod constants {
    pub const APP_NAME: &str = "hyprbinds";
    pub const APP_VERSION: &str = "v0.0.1";

    pub const RE_PATTERN_EXTRACT: &str = "\\((?<cmd>.*),";
    pub const RE_PATTERN_CLEANUP: &str = "\\s{1}+\\.\\.\\s*\" \\+ \"\\s*\\.\\.\\s*\"([^\"]+)\"";

    // TODO: Replace absolute path
    pub const DEFAULT_CONFIG_PATH: &str = "~/.config/hypr/keybinds.lua";

    pub const ARG_CONFIG_PATH: &str = "config-type";
    pub const ARG_CONFIG_TYPE: &str = "config-type";
}
