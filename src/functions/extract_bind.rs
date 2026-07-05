use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::constants;

pub fn extract_bind(line: &str) -> Option<HashMap<&'static str, String>> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(constants::RE_PATTERN).unwrap());

    let caps = re.captures(line)?;
    let kb_raw = &caps[constants::K_KB];
    let desc = match caps.name(constants::K_DESC) {
        Some(m) => m.as_str(),
        _ => constants::DESC_EMPTY,
    };

    let kb_cleaned = kb_raw
        .replace("..", "")
        .replace('"', "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    Some(HashMap::from([
        (constants::K_KB, kb_cleaned),
        (constants::K_DESC, String::from(desc)),
    ]))
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
            (constants::K_KB, String::from("mainMod + R")),
            (constants::K_DESC, String::from(constants::DESC_EMPTY)),
        ]));

        assert_eq!(crate::extract_bind(&test_case), result);
    }

    #[test]
    fn test_extract_bind_multi_key() {
        let test_case: &str = r#"hl.bind(mainMod .. " + " .. "SHIFT" .. " + " .. "R", hl.dsp.exec_cmd("kitty -e yazy"))"#;
        let result = Some(HashMap::from([
            (constants::K_KB, String::from("mainMod + SHIFT + R")),
            (constants::K_DESC, String::from(constants::DESC_EMPTY)),
        ]));

        assert_eq!(crate::extract_bind(&test_case), result);
    }

    #[test]
    fn test_extract_bind_with_description() {
        let test_case: &str = r#"hl.bind(mainMod .. " + " .. "SHIFT" .. " + " .. "R", hl.dsp.exec_cmd("kitty -e yazy"), { description = "Open file manager" })"#;
        let result = Some(HashMap::from([
            (constants::K_KB, String::from("mainMod + SHIFT + R")),
            (constants::K_DESC, String::from("Open file manager")),
        ]));

        assert_eq!(crate::extract_bind(&test_case), result);
    }

    #[test]
    fn test_extract_bind_multi_line_with_description() {
        let test_case: &str = r#"\
            hl.bind(
                mainMod .. " + " .. "SHIFT" .. " + " .. "R",
                hl.dsp.exec_cmd("kitty -e yazy"),
                { description = "Open file manager" }
            )"#;

        let result = Some(HashMap::from([
            (constants::K_KB, String::from("mainMod + SHIFT + R")),
            (constants::K_DESC, String::from("Open file manager")),
        ]));

        assert_eq!(crate::extract_bind(&test_case), result);
    }
}
