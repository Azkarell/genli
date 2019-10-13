extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::Button::Keyboard;
use super::super::Game;
use super::mapgen::MapGen;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;


pub struct MapGame {
    gl: glutin_window::OpenGL,
    gen: MapGen,
    draw: Box<dyn Fn(graphics::Context,&mut opengl_graphics::GlGraphics)>
}

impl Game for MapGame {
    fn play(&mut self) -> Result<(), String> {
        self.gen.restart();
        self.draw = self.gen.next();
        let mut window: Window = WindowSettings::new("MapGame", [600, 600])
            .graphics_api(self.gl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }
            if let Some(r) = e.release_args(){
                if let Keyboard(k) = r {
                    match k {
                        Key::Return => self.draw = self.next(),
                        _ => ()
                    };
                }
            }
        }
        Ok(())
    }
    fn name(&self) -> &str {
        "MapGame"
    }
}

impl dialoguer::Validator for MapGame {
    type Err = String;
    fn validate(&self, text: &str) -> Result<(), String> {
        if super::is_valid_map_arg(text) {
            Ok(())
        } else {
            Err(String::from("Invalid input!"))
        }
    }
}

impl MapGame {
    fn render(&mut self, args: &RenderArgs) {

        let mut gl = GlGraphics::new(self.gl);

        gl.draw(args.viewport(), &self.draw);
    }

    fn next(&mut self) ->  Box<dyn Fn(graphics::Context, &mut opengl_graphics::GlGraphics)>{
        self.gen.next()
    }

    pub fn new(seed: u64, width: u64, height: u64) -> Self {
        let mut gen = MapGen::new(seed, width, height);
        let opengl = OpenGL::V4_5;
        MapGame {
            gl: opengl,
            draw: gen.next(),
            gen: gen
        }
    }
    pub fn from_game_args(seed: u64, args: &Vec<&str>) -> Self {
        MapGame::new(seed, args[0].parse().expect("not a number"), args[1].parse().expect("not a number"))
    }
    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
    }
}


