use std::io;
use std::process;

#[derive(Debug, PartialEq)]
pub struct Command {
    pub command: String,
    pub arguments: Vec<String>,
}

impl Command {
    /// Executes the command and returns a `std::process::ExitStatus` wrapped
    /// in a `std::io::Result`, if the command couldn't be executed
    /// the function will return a `Err`, if it could be executed in the other
    /// hand, `Ok(std::process::ExitStatus)` will be returned
    pub fn execute_command(&self) -> io::Result<process::ExitStatus> {
        // TODO: Return code or `ExitStatus`
        process::Command::new(self.command.clone())
            .args(self.arguments.clone())
            .stdout(process::Stdio::inherit())
            .stdin(process::Stdio::inherit())
            .status()
    }
}

pub fn parse_input(mut input: String) -> Result<Command, ()> {
    input += " ";
    let mut quote_status = false;
    let mut command = String::new();
    let mut foo = String::new();
    let mut args = Vec::new();

    for c in input.chars() {
        // End of the segment
        if c == ' ' && !quote_status {
            if foo.trim() == "" {
                continue;
            }
            if command != "" {
                args.push(foo.clone());
            } else {
                command = foo.clone();
            }
            foo = String::new();
        // Quotes
        } else if c == '\"' {
            quote_status = !quote_status;
        } else {
            foo += &c.to_string();
        }
    }
    // Non closed quotes
    if quote_status && foo != "" {
        return Err(()); // BUG: Doesn't get returned when there are non closed quotes
    }
    Ok(Command {
        command,
        arguments: args,
    })
}
