
pub trait Game {
    fn play(&mut self) -> Result<(),String>;
}

