use std::env;

use super::Runnable;

pub struct Set {}

impl Set {
    pub fn new() -> Self {
        Self{}
    }
}

impl Runnable for Set {
    fn push_args(&mut self, _word: &str) -> () {
        
    }

    fn run(&mut self) -> Result<(), &str> {
        for (key, value) in env::vars() {
            println!("{}={}", key, value);
        }
        Ok(())
    }
}