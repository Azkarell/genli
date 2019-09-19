extern crate clap;
extern crate rand;
extern crate regex;

use clap::{App, Arg, SubCommand};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use regex::{Regex};

mod dice;


fn main() {
    let matches = init_arg_parser();

    let seed: u64 = match matches.value_of("seed") {
        Some(val) => val.parse().unwrap(),
        None => generate_seed(),
    };

    println!("Seed: {}", seed);

    if let Some(mymatch) = matches.subcommand_matches("dice") {
        generate_dices(mymatch, seed);
    }
}

fn generate_seed() -> u64 {
    let mut rng = rand::thread_rng();
    return rng.gen();
}

fn init_arg_parser() -> clap::ArgMatches<'static> {
    let matches = App::new("genli")
        .version("0.1")
        .author("Daniel Schröder")
        .about("Generate everything ;)")
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .help("use specific seed"),
        )
        .subcommand(
            SubCommand::with_name("name").about("Generates a name").arg(
                Arg::with_name("gender")
                    .takes_value(true)
                    .short("g")
                    .long("gender")
                    .help("specifices a specific gender")
                    .validator(|n| {
                        if n == "m" || n == "f" || n == "b" {
                            return Ok(());
                        } else {
                            return Err(String::from("valid arguments: m, f, b [defalut b]"));
                        }
                    })
                    .default_value("b")
                    .required(false),
            ),
        )
        .subcommand(
            SubCommand::with_name("dice").about("rolls a dices").arg(
                Arg::with_name("DICES")
                    .takes_value(true)
                    .validator(|n| dice_args_match(&n))
                    .multiple(true),
            ),
        )
        .get_matches();
    return matches;
}

fn generate_dices(matches: &clap::ArgMatches<'_>, seed: u64) {
    let dices = match matches.values_of("DICES") {
        Some(val) => parse_dices(val),
        None => vec![dice::DiceRoll{ dice_count: 1, dice_sides: 6, addition: 0}],
    };

    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    let mut sum = 0;
    for d in dices {
        let res = d.roll(&mut rng);
        sum += res.get_result();
        println!("{}", res);
    }
    println!("Total: {}",sum);
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

fn parse_dices(val: clap::Values) -> Vec<dice::DiceRoll> {
    let regex = get_dice_regex();

    let mut vec = Vec::new();
    for v in val {
        let iter = regex.captures_iter(v);
        for c in iter {
            let dice_count = c.name("count").map_or(0u64, |m| m.as_str().parse().unwrap());
            let dice_sides = c.name("sides").map_or(0u64, |m| m.as_str().parse().unwrap());
            let addition = c.name("addition").map_or(0u64, |m| m.as_str().parse().unwrap());
            vec.push(dice::DiceRoll{ dice_count: dice_count, dice_sides: dice_sides, addition: addition });
        }
    }

    return vec;
}
