use crate::shell;

#[test]
fn string_parser() {
    let result = shell::parse_input("welcome to \"the league of\" legends".to_string());
    let target = shell::Command {
        cmd: String::from("welcome"),
        args: vec![
            "to".to_string(),
            "the league of".to_string(),
            "legends".to_string(),
        ],
    };
    assert_eq!(Ok(target), result);
}

#[test]
fn string_parser_extra_spaces() {
    let result = shell::parse_input("welcome to   \"the league of\" legends  ".to_string());
    let target = shell::Command {
        cmd: String::from("welcome"),
        args: vec![
            "to".to_string(),
            "the league of".to_string(),
            "legends".to_string(),
        ],
    };
    assert_eq!(Ok(target), result);
}

#[test]
fn string_parser_non_closing_quotes() {
    let result = shell::parse_input("welcome to   \"the league of legends  ".to_string());
    let target = shell::Command {
        cmd: String::from("welcome"),
        args: vec!["to".to_string(), "the league of legends  ".to_string()],
    };
    assert_eq!(Ok(target), Err(()));
}

#[test]
fn string_parser_empty_cmd() {
    let result = shell::parse_input("".to_string());
    let target = shell::Command {
        cmd: "".to_string(),
        args: vec![],
    };
    assert_eq!(Ok(target), result);
}