extern crate console;
extern crate dialoguer;

mod args;
mod game;
use std::error::Error;

fn main() {

    config_terms();
    let ap = args::ArgParser::new();
    let game = ap.get_game();
    match game {
        Some(val) => {
           play_game_in_loop(val)
        }
        None => println_with_color("Game not implemented or no game found!", console::Color::Red)
    }
}

fn play_game_in_loop(mut game: Box<dyn game::Game<Err=String>>){
     let mutgame = game.as_mut();
            loop {
                clear_console();
                match mutgame.play() {
                    Ok(_) => {
                         println!("{} played!", mutgame.name());
                         match ask_play_again() {
                             Ok(val) => { if !val { break } }
                             Err(err) => println!("{}",err)
                         }
                    }
                    Err(err) => println!("{}",err)
                }
            }
}


fn ask_play_again() -> Result<bool,String>  {
    match dialoguer::Confirmation::new().with_text("Play again?").interact() {
        Ok(val) => Ok(val),
        Err(err) => Err(err.description().to_owned())
    }
}

fn config_terms(){
    let term = console::Term::stdout();
    term.set_title("dice");
    console::set_colors_enabled(true);
}

fn clear_console() {
    console::Term::stdout().clear_screen().expect("failed to clear console");
}

fn println_with_color(s: &str, color: console::Color){
    let styled = console::style(s).fg(color);
    println!("{}",styled);
}