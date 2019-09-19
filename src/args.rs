extern crate clap;
extern crate rand;

use rand::{Rng};
use super::game;
use super::dice;

use clap::{App, Arg, SubCommand, AppSettings};


pub struct ArgParser {
    matches: clap::ArgMatches<'static>
}

impl ArgParser {
    pub fn get_seed(&self) -> u64{
    return match self.matches.value_of("seed") {
        Some(val) => val.parse().unwrap(),
        None => generate_seed(),
    };
    }

    pub fn get_game(&self) -> Option<Box<dyn game::Game>> {
        return match self.matches.subcommand_matches("dice") {
            Some(mymatch) => Some(Box::new(dice::DiceGame { dices: generate_dices(mymatch) })),
            None => None
        };
    }
}

pub fn init_arg_parser() -> ArgParser {
    let matches = App::new("genli")
        .version("0.1")
        .author("Daniel Schröder")
        .about("Generate everything ;)")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .validator(|s| { 
                    match s.parse::<u64>() {
                        Ok(_) => Ok(()),
                        Err(_) => Err("must be a number".to_owned())
                    }
                })
                .takes_value(true)
                .help("use specific seed"),
        )
        .subcommand(
            SubCommand::with_name("name").about("Generates a name").arg(
                Arg::with_name("gender")
                    .takes_value(true)
                    .short("g")
                    .long("gender")
                    .help("specifies a gender")
                    .possible_values(&vec!["m","f","b"])
                    .default_value("b")
                    .required(false),
            ),
        )
        .subcommand(
            SubCommand::with_name("dice").about("rolls a dices").arg(
                Arg::with_name("DICES")
                    .takes_value(true)
                    .validator(|n| dice::dice_args_match(&n))
                    .multiple(true),
            ),
        )
        .get_matches();
    return ArgParser{matches: matches};
}


fn generate_seed() -> u64 {
    let mut rng = rand::thread_rng();
    return rng.gen();
}

fn generate_dices(matches: &clap::ArgMatches<'_>) -> Vec<dice::DiceRoll> {
    let dices = match matches.values_of("DICES") {
        Some(val) => dice::parse_dices(val),
        None => vec![dice::DiceRoll{ dice_count: 1, dice_sides: 6, addition: 0}],
    };
    return dices;
}




