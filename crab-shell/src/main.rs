use crab_shell_core::*;

fn main() {
    let mut s = shell::Shell::new();
    //s.prompt_handler = std::boxed::Box::new(random_handler);
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
fn random_handler() -> String {
    "TEST ::: ".to_string()
}
