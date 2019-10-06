use std::fmt;
use super::diceresults::DiceResults;
use rand::{Rng};
use rand::distributions::{Distribution, Uniform};

#[derive(Debug)]
pub struct DiceRoll {
    pub dice_count: i64,
    pub dice_sides: i64,
    pub addition: i64,
    pub sign: i64
}


impl DiceRoll {
    pub fn roll<R: Rng>(&self,rng:&mut R) -> DiceResults {
        let mut dice_results = vec![];
        for _ in 0..self.dice_count {
            let die = Uniform::from(1..=self.dice_sides);
            dice_results.push( die.sample(rng));
        }
        return DiceResults { addition: self.addition, dice_count: self.dice_count, results: dice_results, dice_sides: self.dice_sides, sign: self.sign };
    } 
    pub fn get_sign_as_str(&self) -> &str{
        if self.sign < 0  {
            return "-";
        }else {
            return "+";
        }
    }
}
impl fmt::Display for DiceRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}d{}{}{}", self.dice_count, self.dice_sides, self.get_sign_as_str(), self.addition)
    }
}
