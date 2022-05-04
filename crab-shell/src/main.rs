use colored::Colorize;
use crab_shell_core::*;
use std::boxed;
use std::env;
use whoami;

fn main() {
    let mut s = shell::Shell::new();
    s.prompt_handler = boxed::Box::new(prompt_handler);
    loop {
        let user_input = s.prompt_user();
        let code = s.execute_user_input(command::parse_input(user_input).unwrap());
        if let None = code {
            eprintln!("Command not found.");
        } else {
            continue;
        }
    }
}

// Testing stuff
fn prompt_handler() -> String {
    format!(
        "[{}@{} {}] ",
        whoami::username().green(),
        whoami::hostname().blue(),
        env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .bright_blue()
    )
}
