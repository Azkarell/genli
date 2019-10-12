use super::diceresults::DiceResults;
use super::diceroll::DiceRoll;
use super::super::Game;
use rand::rngs::StdRng;

#[derive(Debug)]
pub struct DiceGame {
    dices: Vec<DiceRoll>,
    rng: StdRng,
    last_results: Vec<DiceResults>,
    name: String
}

impl DiceGame {
    pub fn new(seed: u64, dices: Vec<DiceRoll>) -> Self {
        return DiceGame {
            dices: dices,
            rng: super::super::get_rng(seed),
            last_results: Vec::new(),
            name: "DiceGame".to_owned()
        };
    }

    pub fn use_seed(&mut self, seed: u64) {
        self.rng = super::super::get_rng(seed);
    }

    pub fn is_valid_dice_game(s: &str) -> Result<(), String> {
        return super::dice_args_match(s);
    }

    pub fn from_game_args(seed: u64, args: &Vec<&str>) -> Self {
        let dices = parse_dices(args);
        return DiceGame::new(seed, dices);
    }

    fn roll_dices(&mut self) {
        let mut vec = Vec::new();
        let mut sum = 0;
        for d in &self.dices {
            let roll = d.roll(&mut self.rng);
            let result = roll.get_result();
            sum += result;
            vec.push(roll);
            println!("You've rolled a {} with {}", result, d);
        }
        self.last_results = vec;
        println!("Total: {}", sum);
    }
}

impl Game for DiceGame {
    fn play(&mut self) -> Result<(), String> {
        // self.print_welcome();
        self.roll_dices();
        Ok(())
    }
    fn name(&self) -> &str{
        &self.name
    }
}

impl dialoguer::Validator for DiceGame{
    type Err = String;
    fn validate(&self, text: &str )-> Result<(),String>{
        if super::is_valid_dice_arg(text) {
            Ok(())
        } else {
            Err(String::from("Invalid input!"))
        }
    }
}


fn parse_dices(val: &Vec<&str>) -> Vec<DiceRoll> {
    let regex = super::get_dice_regex();

    let mut vec = Vec::new();
    for v in val {
        let iter = regex.captures_iter(v);
        for c in iter {
            let dice_count = c.name("count").map_or(0i64, |m| m.as_str().parse().unwrap());
            let dice_sides = c.name("sides").map_or(0i64, |m| m.as_str().parse().unwrap());
            let addition = c.name("addition").map_or(0i64, |m| m.as_str().parse().unwrap());
            let sign = c.name("sign").map_or(1i64, |m| match m.as_str() {
                "+" => 1i64,
                "-" => -1i64,
                &_ => 1i64
            });
            vec.push(DiceRoll{ dice_count: dice_count, dice_sides: dice_sides, addition: addition, sign: sign });
        }
    }

    return vec;
}