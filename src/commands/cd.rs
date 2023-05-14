use std::{path::Path, env, panic};


use super::Runnable;

pub struct Cd {
    args: Vec<String>
}

impl Runnable for Cd {
    fn push_args(&mut self, word: &str) -> () {
        self.args.push(word.to_string());
        ()
    }

    fn run(&mut self) -> Result<(), &str> {
        if self.args.len() > 1 {
            return Err("cd: Too many arguments");
        } else if self.args.len() == 0 {
            return Err("cd: missing arguments");
        } else {

            let root: &Path = Path::new("/");
            let path: &str = &self.args[0];
        
            let absolute_path = if path.starts_with("/") {
                Path::new("/").to_path_buf()
            } else {
                env::current_dir().unwrap().join(path)
            };
            
            if !absolute_path.exists() || !absolute_path.is_dir() {
                return Err("cd: path does not exist");
            }

            let canonical_path = absolute_path
                .canonicalize()
                .unwrap_or_else(|_| panic!("cd: failed to canonicalize path {:?}", absolute_path));
                
            if !canonical_path.starts_with(root) {
                panic!("cd: attempted to access a path outside of the root directory!");
            }
            
            env::set_current_dir(canonical_path.as_path()).unwrap();
            env::set_var("PWD", canonical_path.display().to_string());
            Ok(())
        }
    }
}

impl Cd {
    pub fn new() -> Self {
        Self {
            args: vec![]
        }
    }
}