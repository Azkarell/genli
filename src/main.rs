

mod args;
mod game;


fn main() {

    let ap = args::ArgParser::new();
    let game = ap.get_game();
    match game {
        Some(mut val) => match val.as_mut().play() {
            Ok(_) => println!("{} played!", val.name()),
            Err(err) => println!("{}",err)
        }
        None => println!("Game not implemented or no game found!")
    }
}




