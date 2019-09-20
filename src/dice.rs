extern crate rand;
extern crate regex;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::{Distribution, Uniform};
use regex::{Regex};
use std::fmt;

use super::game;

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


impl fmt::Display for DiceRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}d{}+{}", self.dice_count, self.dice_sides, self.addition)
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
       return write!(f,"({}d{}+{} : {})", self.dice_count,self.dice_sides, self.addition, self.get_result());
    }
}


fn is_valid_dice_arg(s: &str) -> bool {
    let regex = get_dice_regex();
    return regex.is_match(&s);
}

fn dice_args_match(s: &str) -> Result<(), String> {
    if is_valid_dice_arg(s) {
        return Ok(());
    } else {
        return Err("valid arguments: e.g 3d6".to_string());
    }
}

fn get_dice_regex() -> Regex {
    let regex = Regex::new("((?P<count>[1-9][0-9]*+)(w|d)(?P<sides>[1-9][0-9]*+))(\\+(?P<addition>[1-9][0-9]*+))*+").unwrap();
    return regex;
}

fn parse_dices(val: &Vec<&str>) -> Vec<DiceRoll> {
    let regex = get_dice_regex();

    let mut vec = Vec::new();
    for v in val {
        let iter = regex.captures_iter(v);
        for c in iter {
            let dice_count = c.name("count").map_or(0u64, |m| m.as_str().parse().unwrap());
            let dice_sides = c.name("sides").map_or(0u64, |m| m.as_str().parse().unwrap());
            let addition = c.name("addition").map_or(0u64, |m| m.as_str().parse().unwrap());
            vec.push(DiceRoll{ dice_count: dice_count, dice_sides: dice_sides, addition: addition });
        }
    }

    return vec;
}


pub struct DiceGame {
    dices: Vec<DiceRoll>,
    rng: StdRng,
    last_results: Vec<DiceResults>
}

impl DiceGame {
    pub fn new(seed: u64, dices: Vec<DiceRoll>) -> Self{
        return DiceGame{ dices: dices, rng: get_rng(seed), last_results: Vec::new()}
    }

    pub fn use_seed(&mut self,seed: u64){
        self.rng = get_rng(seed);
    }

    pub fn is_valid_dice_game(s: &str) -> Result<(),String>{
        return dice_args_match(s);
    }

    pub fn from_game_args(seed: u64, args: &Vec<&str>) -> Self {
        let dices = parse_dices(args);
        return DiceGame::new(seed,dices);
    }

    fn print_welcome(&self){
        println!("Your playing random dice game with: ");
        for d in &self.dices {
            println!("{}", d )
        }
    }

    fn roll_dices(&mut self)
    {
        let mut vec = Vec::new();
        let mut sum = 0;
        for d in&self.dices{
            let roll = d.roll(&mut self.rng);
            let result = roll.get_result();
            sum += result;
            vec.push(roll);
            println!("You've rolled a {} with {}",result,d);
        }
        println!("Total: {}", sum);
    }
}


impl game::Game for DiceGame {
    fn play(&mut self) -> Result<(),String>{
        self.print_welcome();
        self.roll_dices();
        return Ok(());
    }
}

fn get_rng(seed: u64) -> StdRng{
    return <StdRng as SeedableRng>::seed_from_u64(seed);
}