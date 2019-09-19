

mod args;
mod game;
mod dice;

fn main() {

    let ap = args::init_arg_parser();
    let seed = ap.get_seed();
    let game = ap.get_game();
    match game {
        Some(val) => match val.as_ref().play(seed) {
            Ok(res) => res.as_ref().print(),
            Err(err) => println!("{}",err)
        }
        None => println!("No game selected or implemented!")
    }
}




