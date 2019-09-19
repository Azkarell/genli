extern crate rand;

use rand::{Rng};
use rand::distributions::{Distribution, Uniform};

use std::fmt;

pub struct DiceRoll {
    pub dice_count: u64,
    pub dice_sides: u64,
    pub addition: u64
}


impl DiceRoll {
    pub fn roll<R: Rng>(&self,rng:&mut R) -> DiceResults {
        let mut dice_results = vec![];
        for _ in 0..self.dice_count {
            let die = Uniform::from(1..=self.dice_sides);
            dice_results.push( die.sample(rng));
        }
        return DiceResults { addition: self.addition, dice_count: self.dice_count, results: dice_results, dice_sides: self.dice_sides };
    } 
}



pub struct DiceResults {
    pub results: Vec<u64>,
    pub dice_count: u64,
    pub dice_sides: u64,
    pub addition: u64
}

impl DiceResults {
    pub fn get_result(&self) -> u64 {
        let res : u64 = self.results.iter().sum();
        return self.addition + res;
    }
}

impl fmt::Display for DiceResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       return write!(f,"(count: {}, sides: {}, addition: {}, result: {})", self.dice_count,self.dice_sides, self.addition, self.get_result());
    }
}