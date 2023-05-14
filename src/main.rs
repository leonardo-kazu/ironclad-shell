use std::panic;

use ironclad::Ironclad;

mod command_list;
mod commands;
mod error_handler;
mod ironclad;
mod my_error;
mod parser;
mod read_stdin;
mod write_stdout;

pub const SHELL_NAME: &str = "-ironclad:";

fn main() {
    panic::set_hook(Box::new(panic_hook));
    Ironclad::new().init();
}


fn panic_hook(info: &panic::PanicInfo) {
    eprintln!("-ironclad: {}", info);
}