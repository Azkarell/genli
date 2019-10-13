extern crate clap;
extern crate rand;

use super::game;
use super::game::dice;
use super::game::map;
use clap::{App, Arg, SubCommand, AppSettings};
use std::iter::FromIterator;

pub struct ArgParser {
    matches: clap::ArgMatches<'static>
}

impl ArgParser {


    pub fn get_game(&self) -> Option<Box<dyn game::Game<Err=String>>> {
        let seed = match self.matches.value_of("seed") {
            Some(val) => val.parse().unwrap(),
            None => super::game::generate_seed()
        };
        if let Some(dice_args) = self.matches.subcommand_matches("dice"){
            return Some(Box::new(dice::dicegame::DiceGame::from_game_args(seed, &map_dice_game_args(&dice_args))));
        }
        if let Some(map_args) = self.matches.subcommand_matches("map"){
            return Some(Box::new(map::mapgame::MapGame::from_game_args(seed, &map_map_game_args(&map_args))))
        }

        None
    }

    pub fn new() -> Self {
        return init_arg_parser();
    }
}

fn init_arg_parser() -> ArgParser {
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
           generate_name_args()
        )
        .subcommand(
            generate_dice_args()
        )
        .subcommand(
            generate_map_args()
        )
        .get_matches();
    return ArgParser{matches: matches};
}

fn generate_dice_args<'a,'b>() -> clap::App<'a,'b> {
  return SubCommand::with_name("dice").about("rolls a dices").arg(
                Arg::with_name("DICES")
                    .takes_value(true)
                    .validator(|n| dice::dicegame::DiceGame::is_valid_dice_game(&n))
                    .multiple(true),
            )
}

fn generate_name_args<'a,'b>() -> clap::App<'a,'b> {
    SubCommand::with_name("name").about("Generates a name").arg(
                Arg::with_name("gender")
                    .takes_value(true)
                    .short("g")
                    .long("gender")
                    .help("specifies a gender")
                    .possible_values(&vec!["m","f","b"])
                    .default_value("b")
                    .required(false),
            )
}

fn generate_map_args<'a,'b>() -> clap::App<'a,'b> {
    SubCommand::with_name("map")
            .about("Generates a map")
            .arg(
                Arg::with_name("SIZE")
                    .takes_value(true)
                    .validator(|n| map::is_valid_map_game(&n))
                    .max_values(2)
                    .min_values(2)
                    .multiple(true),
            )
}



fn map_dice_game_args<'a>(matches: &'a clap::ArgMatches<'a>) -> Vec<&'a str> {
    return match matches.values_of("DICES") {
        Some(val) => Vec::from_iter(val),
        None => vec!["1d6"]
    };
}

fn map_map_game_args<'a>(matches: &'a clap::ArgMatches<'a>) -> Vec<&'a str> {
    match matches.values_of("SIZE") {
        Some(val) => Vec::from_iter(val),
        None => vec!["600", "600"]
    }
}




