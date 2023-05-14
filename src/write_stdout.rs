use std::{
    env,
    io::{self, Write},
};

pub struct WriteStdout {}

impl WriteStdout {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write_ps1(&self) -> Result<(), io::Error> {
        let ps1 = env::var("MYPS1").unwrap_or_default();
        print!("{}", ps1);
        io::stdout().flush()?;
        Ok(())
    }
}
