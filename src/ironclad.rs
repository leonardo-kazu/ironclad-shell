use std::env;

use crate::command_list::CommandList;
use crate::error_handler::ErrorHandler;
use crate::read_stdin::ReadStdin;
use crate::write_stdout::WriteStdout;

// The main corpus of ironclad
pub struct Ironclad {
    reader: ReadStdin,
    error_handler: ErrorHandler,
    write_stdout: WriteStdout,
}

impl Ironclad {
    pub fn new() -> Self {
        Self {
            reader: ReadStdin::new(),
            error_handler: ErrorHandler::new(),
            write_stdout: WriteStdout::new(),
        }
    }

    pub fn init(&mut self) -> () {
        // Signal treatment using libc
        unsafe {
            libc::signal(libc::SIGINT, Self::handle_interrupt as libc::sighandler_t);
        }
        env::set_var(
            "MYPATH",
            env::var("PATH").unwrap_or(String::from("").into()),
        );
        env::set_var("MYPS1", String::from("tecii$ "));

        self.run();
    }

    fn run(&mut self) {
        loop {
            self.write_stdout.write_ps1().unwrap_or_else(|err| {
                self.error_handler.handle_error(err.to_string());
                ()
            });

            let input_list = self.reader.read().unwrap_or_else(|err| {
                self.error_handler.handle_error(err.to_string());
                [String::new()].into()
            });

            if input_list.is_empty() {
                continue;
            }

            let mut command_list: CommandList = CommandList::new();

            command_list
                .prepare(input_list)
                .unwrap_or_else(|err| self.error_handler.handle_error(err.to_string()));

            command_list.set();

            command_list
                .go()
                .unwrap_or_else(|err| self.error_handler.handle_error(err.to_string()));
        }
    }

    extern "C" fn handle_interrupt(_sig: libc::c_int) {
        
    }
}

