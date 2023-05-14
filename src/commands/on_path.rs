use crate::my_error::MyError;

use std::{env, path::Path};
use std::error::Error;
use std::fs::File;
use std::process::{Child, Command, Stdio};

enum CommandPossibilities {
    Undefined,
    Defined(Command),
}

pub struct OnPath {
    args: Vec<String>,
    command: Command,
}

impl OnPath {
    pub fn new(name: &str) -> Result<Self, MyError> {
        let paths = env::var("MYPATH").unwrap_or_else(|_| String::new());
        let mut command_real: CommandPossibilities = CommandPossibilities::Undefined;
        if Path::new(name).exists() {
            command_real = CommandPossibilities::Defined(Command::new(name));
        } else {
            for path in env::split_paths(&paths) {
                let mut full_path = path.clone();
                full_path.push(name);
                if full_path.as_path().exists() {
                    command_real = CommandPossibilities::Defined(Command::new(full_path));
                    break;
                }
            }
        }
        if let CommandPossibilities::Defined(e) = command_real {
            Ok(Self {
                command: e,
                args: vec![],
            })
        } else {
            Err(MyError::new(format!("{}: command not found", name)))
        }
    }

    pub fn push_args(&mut self, word: &str) -> () {
        self.args.push(word.to_string());
        ()
    }

    pub fn set_args(&mut self) -> () {
        self.command.args(self.args.clone());
        ()
    }

    pub fn run(
        &mut self,
        stdin: &String,
        stdout: &String,
        stderr: &String,
        piped: bool,
        command: Option<Child>,
        background: bool
    ) -> Result<Child, Box<dyn Error>> {
        if !piped {
            if stdin.is_empty() {
                if let Some(c) = command {
                    self.command.stdin(c.stdout.unwrap());
                }
            } else {
                self.command.stdin(File::create(stdin)?);
            }

            if !stdout.is_empty() {
                self.command.stdout(File::create(stdout)?);
            } else {
                if background {

                    self.command.stdout(Stdio::piped());
                }
            }
            if !stderr.is_empty() {
                self.command.stderr(File::create(stderr)?);
            } else {
                if background {

                    self.command.stderr(Stdio::piped());
                }
            }
        } else {
            if let Some(c) = command {
                self.command.stdin(c.stdout.unwrap());
                self.command.stdout(Stdio::piped());
                if !stderr.is_empty() {
                    self.command.stderr(File::create(stderr)?);
                }
            } else {
                self.command.stdout(Stdio::piped());
                if !stderr.is_empty() {
                    self.command.stderr(File::create(stderr)?);
                }
                if !stdin.is_empty() {
                    self.command.stdin(File::open(stdin)?);
                }
            }
        }
        Ok(self.command.spawn()?)
    }
}

