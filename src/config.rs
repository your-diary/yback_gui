use json::{self, JsonValue};
use regex::Regex;
use std::fs;
use std::process::{Command, Output};

use super::backup::BackupConfig;

#[derive(PartialEq)]
pub enum IsDryrunMode {
    YES,
    NO,
}

fn jsonvalue_to_string(j: &JsonValue) -> String {
    command_substitution(j.as_str().unwrap().to_string())
}

fn jsonvalue_to_string_array(j: &JsonValue) -> Option<Vec<String>> {
    if let JsonValue::Array(array) = j {
        Some(
            array
                .iter()
                .map(|a| command_substitution(a.as_str().unwrap().to_string()))
                .collect::<Vec<_>>(),
        )
    } else {
        None
    }
}

//Perform command substitution for the given string.
//We have tests for this method. See `mod command_substitution`.
//
//FIXME:
//  Currently, only one command substitution is supported, though it can be nested:
//  OK: `a$(...)b` (single)
//  OK: `a$(...$(...)...)b` (single, nested)
//  NG: `a$(...)b$(...)c` (double)
fn command_substitution(mut s: String) -> String {
    let regex = Regex::new(r"\$\((.*)\)").unwrap();
    match regex.captures(&s) {
        Some(c) => {
            let command = c.get(1).unwrap().as_str();
            let range = c.get(0).unwrap().range();
            let res: Output = Command::new("bash").args(["-c", command]).output().unwrap();
            let res: String = String::from_utf8(res.stdout).unwrap().trim().to_string();
            s.replace_range(range, &res);
        }
        None => (),
    }
    s
}

pub fn read_config_file(is_dryrun_mode: IsDryrunMode) -> Option<Vec<BackupConfig>> {
    let config_file = "./config.json";

    let json_string: String = fs::read_to_string(config_file).unwrap();
    let json_object: JsonValue = json::parse(&json_string).unwrap();

    if let JsonValue::Object(o) = json_object {
        if let JsonValue::Array(config_list) = o.get("config_list").unwrap() {
            return Some(
                config_list
                    .iter()
                    .filter_map(|config: &JsonValue| {
                        if let JsonValue::Object(o) = config {
                            let name: String = jsonvalue_to_string(o.get("name").unwrap());
                            let from: Vec<String> =
                                jsonvalue_to_string_array(o.get("from").unwrap()).unwrap();
                            let to: String = jsonvalue_to_string(o.get("to").unwrap());
                            let mut options: Vec<String> =
                                jsonvalue_to_string_array(o.get("options").unwrap()).unwrap();
                            if (is_dryrun_mode == IsDryrunMode::YES) {
                                options.push(String::from("--dry-run"))
                            }
                            Some(BackupConfig::new(name, from, to, options))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>(),
            );
        }
    }

    None
}

#[cfg(test)]
mod command_substitution {
    #[test]
    fn test01() {
        let input: String = String::from("a$(echo hello)b");
        let expected = "ahellob";
        let actual = super::command_substitution(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test02() {
        let input: String = String::from("a$(echo $(echo hello))b");
        let expected = "ahellob";
        let actual = super::command_substitution(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test03() {
        let input: String = String::from("a$(echo hello)b$(echo world)c");
        let expected = "ac";
        let actual = super::command_substitution(input);
        assert_eq!(expected, actual);
    }
}
