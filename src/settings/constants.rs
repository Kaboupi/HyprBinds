pub mod constants {
    pub const APP_NAME: &str = "hyprbinds";
    pub const APP_VERSION: &str = "v0.0.1";

    pub const RE_PATTERN: &str =
        "hl\\.bind\\((?<mod>\\w+)\\s*\\.\\.\\s*\"\\s*\\+\\s*\"\\s*\\.\\.\\s*\"(?<key>\\w+)\"";

    // TODO: Replace absolute path
    pub const DEFAULT_CONFIG_PATH: &str = "~/.config/hypr/keybinds.lua";

    pub const ARG_CONFIG_PATH: &str = "config-type";
}
