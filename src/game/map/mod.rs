
pub mod mapgame; 

mod map;
mod mapgen;

pub fn is_valid_map_arg(s: &str) -> bool{
    s.parse::<u64>().is_ok()
}

pub fn is_valid_map_game(s: &str) -> Result<(), String> {
    if is_valid_map_arg(s) {
        return Ok(());
    } else {
        return Err("valid arguments: e.g 200 200".to_string());
    }
}

