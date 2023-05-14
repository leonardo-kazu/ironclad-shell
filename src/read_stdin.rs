use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

use crate::parser::Parser;

pub struct ReadStdin {
    parser: Parser,
}

impl ReadStdin {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }

    pub fn read(&self) -> Result<Vec<String>, io::Error> {
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                std::process::exit(0);
            }
            Ok(_) => {
                let history = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .read(true)
                    .truncate(false)
                    .open(".history")?;
        
                let reader = BufReader::new(&history);
        
                let lines: Vec<String> = reader.lines().take(49).map(|l| l.unwrap()).collect();
        
                if lines.len() >= 49 {
                    let truncate_len = lines.iter().fold(0, |acc, line| acc + line.len() + 1);
                    history.set_len(truncate_len as u64)?;
                }
        
                drop(history);
        
                let history = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(".history")?;
        
                let mut writer = BufWriter::new(&history);
        
                writer.write_all(&input.as_bytes())?;
        
                let input_list = self.parser.parse(&mut input);
                Ok(input_list)
            }
            Err(error) => Err(error),
        }
    }
}
