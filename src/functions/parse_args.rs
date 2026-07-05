use crate::constants;
use clap::{App, Arg};

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
    use crate::constants;

    #[test]
    fn test_args_extraction_empty() {
        let test_case_buf = crate::parse_args();
        let test_case = test_case_buf
            .value_of(constants::ARG_CONFIG_PATH)
            .unwrap_or_else(|| "test_fallback");

        assert_eq!(test_case, "test_fallback");
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
