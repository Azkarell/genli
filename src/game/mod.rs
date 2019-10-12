use rand::rngs::StdRng;
use rand::{SeedableRng};

pub trait Game : dialoguer::Validator {
    fn name(&self) -> &str;
    fn play(&mut self) -> Result<(),Self::Err>;
}

fn get_rng(seed: u64) -> StdRng {
    return <StdRng as SeedableRng>::seed_from_u64(seed);
}

pub mod dice;
pub mod name;
pub mod map;