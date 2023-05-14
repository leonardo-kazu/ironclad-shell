use lazy_static::lazy_static;
use regex::RegexSet;
use std::io::{Read, self, Write};
use std::{error::Error, process::Child};

use crate::commands::cd::Cd;
use crate::commands::on_path::OnPath;
use crate::commands::export::Export;
use crate::commands::set::Set;
use crate::commands::Runnable;




enum CommandPossibilities {
    Path(OnPath),
    Shell(Box<dyn Runnable>)
}

pub struct CommandList {
    command_list: Vec<CommandPossibilities>,
    stdin: String,
    stdout: String,
    stderr: String,
    piping: bool,
    background: bool,
}

impl CommandList {
    pub fn new() -> Self {
        Self {
            command_list: vec![],
            stdin: String::new(),
            stdout: String::new(),
            stderr: String::new(),
            piping: false,
            background: false,
        }
    }

    pub fn prepare(&mut self, input_list: Vec<String>) -> Result<(), Box<dyn Error>> {
        // Okay so a quick roundup next refers to what the next word is
        // next = 0 -> is a command
        // next = 1 -> is a stdin file
        // next = 2 -> is a stdout file
        // next = 3 -> is a stderr fileself.stderr = String::from(word);

        // next = 4 -> is an argument for the command
        let mut next: i32 = 0;
        let mut command_list_index: usize = 0;
        for (i, word) in input_list.iter().enumerate() {
            if i == input_list.len() - 1 {
                if word == "&" {
                    self.background = true;
                    break;
                } 
            }

            if REDIRECT_PIPE.is_match(word) {
                let matches: regex::SetMatches = REDIRECT_PIPE.matches(word);
                if matches.matched(0) {
                    self.piping = true;
                    next = 0;
                    command_list_index += 1;
                } else if matches.matched(1) {
                    next = 1;
                } else if matches.matched(2) {
                    next = 2;
                } else {
                    next = 3;
                }
            } else if next == 1 {
                self.stdin = word.to_string();
                next = 0;
                command_list_index += 1;
            } else if next == 2 {
                next = 0;
                self.stdout = word.to_string();
                command_list_index += 1;
            } else if next == 3 {
                self.stderr = word.to_string();
                next = 0;
                command_list_index += 1;
            } else if next == 0 {
                if IRONCLAD_COMMANDS.is_match(word) {
                    let matches: regex::SetMatches = IRONCLAD_COMMANDS.matches(word);
                    if matches.matched(0) {
                        let command = Export::new();
                        self.command_list.push(CommandPossibilities::Shell(Box::new(command)));
                    } else if matches.matched(1) {
                        let command = Set::new();
                        self.command_list.push(CommandPossibilities::Shell(Box::new
                        (command)));
                    } else if matches.matched(2) {
                        let command = Cd::new();
                        self.command_list.push(CommandPossibilities::Shell(Box::new(command)));

                    } else if matches.matched(3) {
                        std::process::exit(0);
                    } else if matches.matched(4) {

                    } else if matches.matched(5) {

                    } else if matches.matched(6) {

                    } else if matches.matched(7) {

                    }
                    next = 4;
                } else {
                    let command = OnPath::new(word)?;
                    self.command_list.push(CommandPossibilities::Path(command));
                    next = 4;
                }
            } else {
                if let CommandPossibilities::Path( ref mut cp) = self.command_list[command_list_index] {
                    cp.push_args(word);
                } else if let CommandPossibilities::Shell(ref mut cp) = self.command_list[command_list_index] {
                    cp.push_args(word)
                }
            }
        }
        Ok(())
    }
   

   pub fn set(&mut self) -> () {
    for command in self.command_list.iter_mut() {
        if let CommandPossibilities::Path(cp) = command {
            cp.set_args();
        }
    }
   }

    pub fn go(&mut self) -> Result<(), Box<dyn Error>> {
        if self.piping && self.background {
            todo!();
        } else if self.piping && !self.background {
            let len: usize = self.command_list.len();
            let mut before: Option<Child> = None;
            for (i, command) in self.command_list.iter_mut().enumerate() {
                if let CommandPossibilities::Path(cp) = command {
                if i == 0 {
                    before = Some(cp.run(
                        &self.stdin,
                        &String::new(),
                        &String::new(),
                        true,
                        None,
                        false
                    )?);
                } else if i == len - 1 {
                    if let Some(e) = before {
                        before = Some(cp.run(
                            &String::new(),
                            &self.stdout,
                            &self.stderr,
                            false,
                            Some(e),
                            false
                        )?);
                    }
                    if let Some(e) = before.as_mut() {
                        e.wait()?;
                    }
                } else {
                    if let Some(e) = before {
                        before = Some(cp.run(
                            &String::new(),
                            &String::new(),
                            &String::new(),
                            true,
                            Some(e),
                            false
                        )?);
                    }
                }
            }
            }
            Ok(())
        } else if !self.piping && self.background {
            if let CommandPossibilities::Path(ref mut cp) = self.command_list[0] {
                let mut child = cp.run(&self.stdin, &self.stdout, &self.stderr, false, None, true)?;
                std::thread::spawn(move || {
                    let stdout = child.stdout.as_mut().unwrap();
                    let stderr = child.stderr.as_mut().unwrap();

                    let mut stdout_data = vec![];
                    let mut stderr_data = vec![];

                    stdout.read_to_end(&mut stdout_data).unwrap();
                    stderr.read_to_end(&mut stderr_data).unwrap();

                    print!("{}", String::from_utf8_lossy(&stdout_data));
                    io::stdout().flush().unwrap();
                    eprint!("{}", String::from_utf8_lossy(&stderr_data));
                    io::stderr().flush().unwrap();
                });
                Ok(())
            } else {
                Ok(())
            }
        } else if !self.command_list.is_empty() {
            if let CommandPossibilities::Path(ref mut cp) = self.command_list[0] {
                let mut child = cp
                .run(&self.stdin, &self.stdout, &self.stderr, false, None, false)?;
                child.wait()?;
        } else if let CommandPossibilities::Shell(ref mut cp) = self.command_list[0] {
            cp.run()?;
        }
            Ok(())
        } else {
            Ok(())
        }
    }
}

lazy_static! {
    static ref REDIRECT_PIPE: RegexSet = RegexSet::new(&[
        r"^\|$",        // pipe         = 0
        r"^<$",         // stdin        = 1
        r"^(>|1>)$",    // stdout       = 2
        r"^(2>)$",      // stderr       = 3
    ]).unwrap();

    static ref IRONCLAD_COMMANDS: RegexSet = RegexSet::new(&[
        r"^(export)$",
        r"^(set)$",
        r"^(cd)$",
        r"^(exit)$",
        r"^(history)$",
        r"^(kill)$",
        r"^(jobs)$",
        r"^(fg)$",
    ]).unwrap();
}
