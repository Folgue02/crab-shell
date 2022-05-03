use super::command;
use super::variables;
use rustyline;
use std::boxed::Box;
use std::env;
use std::io;
use std::process;

pub struct Shell {
    pub rl: rustyline::Editor<()>,
    pub prompt_handler: Box<dyn Fn() -> String>,
    pub vars: variables::Variables,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            rl: rustyline::Editor::<()>::new(),
            prompt_handler: Box::new(Self::default_prompt_handler),
            vars: variables::Variables::new(),
        }
    }
    /// Reads user's `stdin` and returns it as a `String`
    pub fn prompt_user(&mut self) -> String {
        let readline = self.rl.readline(&(self.prompt_handler)()[..]); // TODO: Continue here, how to call a field that contains a function?
        match readline {
            Ok(userinput) => {
                self.rl.add_history_entry(&userinput);
                userinput
            }
            Err(message) => panic!("{}", format!("prompt_user error> {}", message)),
        }
    }

    /// Executes the user input, as a builtin command or a shell command, and returns a
    /// `Some(code)` in that the command could be found, otherwise it will return a `None`
    pub fn execute_user_input(&mut self, user_input: command::Command) -> Option<i32> {
        // Empty command
        if user_input.command == "" {
            return None;
        // Builtin command
        } else if let Some(code) = self.execute_as_builtin(&user_input) {
            return Some(code);
        } else {
            // Execute as shell command
            return match user_input.execute_command() {
                Ok(status) => Some(status.code().unwrap()),
                Err(_) => None,
            };
        }
    }

    pub fn default_prompt_handler() -> String {
        format!("{}) ", std::env::current_dir().unwrap().to_str().unwrap())
    }
    // ************* BUILT-IN COMMANDS *************

    /// Decides what builtin command to execute (*if there is one related to the input, other wise `None`
    /// will be returned*), and returns `Some(code)`.
    pub fn execute_as_builtin(&mut self, user_input: &command::Command) -> Option<i32> {
        let command = user_input.command.as_str();
        return match command {
            // TODO: Create a more dynamic way of implementing builtin commands
            "cd" => Some(self.change_dir(user_input)),
            "exit" => Some(self.exit(user_input)),
            "setvar" => Some(self.set_custom_var(user_input)),
            "unsetvar" => Some(self.unset_custom_var(user_input)),
            "refenvvar" => Some(self.refresh_env_var(user_input)), // Change alias?
            "listvar" => Some(self.list_vars(user_input)),
            _ => None,
        };
    }

    pub fn exit(&self, ctx: &command::Command) -> i32 {
        // TODO: Polish
        let code = if ctx.arguments.len() > 0 {
            match ctx.arguments[0].clone().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Cannot parse the code in the arguments (it has to be an integer)");
                    return 1;
                }
            }
        } else {
            0
        };
        println!("Exiting with code {}...", code);
        process::exit(code);
    }

    pub fn change_dir(&mut self, ctx: &command::Command) -> i32 {
        let target = if ctx.arguments.len() == 1 {
            ctx.arguments[0].clone()
        } else if ctx.arguments.len() > 1 {
            ctx.arguments.join("/")
        } else {
            env::var("HOME").unwrap()
        };

        return match env::set_current_dir(&target) {
            Ok(_) => 0,
            Err(error) => match error.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("Directory '{}' doesn't exist.", target);
                    1
                }
                io::ErrorKind::PermissionDenied => {
                    eprintln!("You lack permissions to enter in '{}'", target);
                    1
                }
                _ => {
                    eprintln!("Unhandled error of type: {}", error);
                    1
                }
            },
        };
    }
    pub fn set_custom_var(&mut self, ctx: &command::Command) -> i32 {
        let (key, value) = if ctx.arguments.len() < 2 {
            eprintln!("Not enough arguments, you have to specify {{key}} and {{value}}");
            return 1;
        } else {
            (ctx.arguments[0].clone(), ctx.arguments[1..].join(":"))
        };
        self.vars.set_var(key, value);
        0
    }
    pub fn unset_custom_var(&mut self, ctx: &command::Command) -> i32 {
        let mut code = 0;
        if ctx.arguments.len() < 1 {
            eprintln!("Not enough arguments, you have to specify {{key}}");
            return 1;
        } else {
            for v in &ctx.arguments {
                match self.vars.unset_var(v) {
                    Ok(_) => (),
                    Err(_) => {
                        eprintln!("Cannot unset '{}' because it doesn't exist.", v);
                        code = 1;
                    }
                }
            }
        }
        code
    }
    pub fn refresh_env_var(&mut self, _ctx: &command::Command) -> i32 {
        self.vars.refresh_env_vars();
        0
    }
    pub fn list_vars(&mut self, _ctx: &command::Command) -> i32 {
        println!("CUSTOM VARIABLES:");
        for custom_var in &self.vars.custom_vars {
            println!("\t{}={}", custom_var.0, custom_var.1);
        }
        println!("ENVIRONMENT'S VARIABLES:");
        for custom_var in &self.vars.env_vars {
            println!("\t{}={}", custom_var.0, custom_var.1);
        }
        0
    }
}
