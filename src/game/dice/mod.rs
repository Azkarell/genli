extern crate rand;
extern crate regex;


use regex::Regex;

pub mod dicegame;
mod diceresults;
mod diceroll;

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
    let regex = Regex::new("((?P<count>[1-9][0-9]*+)(w|d)(?P<sides>[1-9][0-9]*+))((?P<sign>\\+|-)(?P<addition>[1-9][0-9]*+)){0,1}").unwrap();
    return regex;
}




