
pub trait GameResult {

    fn print(&self);

}

pub trait Game {
    fn play(&self, seed: u64) -> Result<Box<dyn GameResult>,String>;
}

