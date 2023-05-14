use std::env;
use super::Runnable;

pub struct Export {
    args: Vec<String>
}

impl Runnable for Export {
  fn push_args(&mut self, word: &str) -> () {
      self.args.push(word.to_string());
      ()
  }

  fn run(&mut self) -> Result<(), &str>{
      for word in self.args.iter_mut() {
        if let Some(x) = word.find('=') {
            let (env, value) = word.split_at_mut(x);
            let value = value.replace("=", "");
            env::set_var(env, value);
        } else {
            return Err("export: wrong arguments")
        }
      }
      Ok(())
  }
}

impl Export {
    pub fn new() -> Self {
        Self {
            args: vec![]   
        }
    }
}