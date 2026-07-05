use clap::{App, Arg};
use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::constants;

pub fn extract_bind(line: &str) -> Option<HashMap<&'static str, String>> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(constants::RE_PATTERN).unwrap());

    let caps = re.captures(line)?;
    let raw_cmd = &caps["raw_cmd"];
    let raw_desc = match caps.name("desc") {
        Some(m) => m.as_str(),
        _ => constants::NO_DESCRIPTION,
    };

    let cleaned = raw_cmd
        .replace("..", "")
        .replace('"', "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    Some(HashMap::from([
        ("kb", cleaned),
        ("desc", String::from(raw_desc)),
    ]))
}

pub fn parse_args() -> clap::ArgMatches<'static> {
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

#[cfg(test)]
mod test {
    use crate::settings::constants::constants;
    use std::collections::HashMap;

    #[test]
    fn test_extract_bind_single_key() {
        let test_case: &str =
            r#"hl.bind(mainMod .. " + " .. "R", hl.dsp.exec_cmd("kitty -e yazy"))"#;
        let result = Some(HashMap::from([
            ("kb", String::from("mainMod + R")),
            ("desc", String::from(constants::NO_DESCRIPTION)),
        ]));

        assert_eq!(crate::extract_bind(&test_case), result);
    }

    #[test]
    fn test_extract_bind_multi_key() {
        let test_case: &str = r#"hl.bind(mainMod .. " + " .. "SHIFT" .. " + " .. "R", hl.dsp.exec_cmd("kitty -e yazy"))"#;
        let result = Some(HashMap::from([
            ("kb", String::from("mainMod + SHIFT + R")),
            ("desc", String::from(constants::NO_DESCRIPTION)),
        ]));

        assert_eq!(crate::extract_bind(&test_case), result);
    }

    #[test]
    fn test_extract_bind_with_description() {
        let test_case: &str = r#"hl.bind(mainMod .. " + " .. "SHIFT" .. " + " .. "R", hl.dsp.exec_cmd("kitty -e yazy"), { description = "Open file manager" })"#;
        let result = Some(HashMap::from([
            ("kb", String::from("mainMod + SHIFT + R")),
            ("desc", String::from("Open file manager")),
        ]));

        assert_eq!(crate::extract_bind(&test_case), result);
    }

    #[test]
    fn test_args_extraction_empty() {
        let test_case_buf = crate::parse_args();
        let test_case = test_case_buf
            .value_of(constants::ARG_CONFIG_PATH)
            .unwrap_or_else(|| constants::DEFAULT_CONFIG_PATH);

        assert_eq!(test_case, constants::DEFAULT_CONFIG_PATH);
    }

    #[test]
    fn test_args_extraction_non_existent() {
        let expected_fallback: &str = "Hello, World!";
        let test_case_buf = crate::parse_args();
        let test_case = test_case_buf
            .value_of("some-random-arg")
            .unwrap_or_else(|| expected_fallback);

        assert_eq!(test_case, expected_fallback);
    }
}
