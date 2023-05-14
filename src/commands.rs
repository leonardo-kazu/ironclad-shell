pub mod set;
pub mod on_path;
pub mod export;
pub mod cd;

pub trait Runnable {
    fn push_args(&mut self, word: &str) -> ();

    fn run(&mut self) -> Result<(), &str>;
}
