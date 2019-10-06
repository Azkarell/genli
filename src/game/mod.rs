pub trait Game {
    fn name(&self) -> &str;
    fn play(&mut self) -> Result<(),String>;
}

pub mod dice;