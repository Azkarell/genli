extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use core::cmp::Ordering::Equal;
use super::super::Game;
use super::map::{Map,TileData};
use super::mapgen::MapGen;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

const WHITE: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct MapGame {
    gl: glutin_window::OpenGL,
    mapdata: Map,
}

impl Game for MapGame {
    fn play(&mut self) -> Result<(), String> {
        let mut window: Window = WindowSettings::new("spinning-square", [600, 600])
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
        use graphics::*;

        let mut gl = GlGraphics::new(self.gl);


        let width = self.mapdata.width();
        let height = self.mapdata.height();

        let square_size_x = args.window_size[0] / width as f64;
        let square_size_y = args.window_size[1] / height as f64;


        let mut floats = vec![square_size_x, square_size_y ];
        floats.sort_by(|a,b| a.partial_cmp(b).unwrap_or(Equal));
        let size = floats[0];
        let square = rectangle::square(0.0, 0.0, size);

        let mut map = self.mapdata.clone();

        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);
            for x in 0 .. width {
                for y in 0 .. height {
                let transform = c
                            .transform
                            .trans(x as f64 * size , y as f64 * size);
                        let color = match map.tile_data(x,y){
                           TileData::Valid{ color } => color,
                           _ => panic!()
                        };
                        // Draw a box rotating around the middle of the screen.
                        rectangle(color, square, transform, gl);
                }
            }
        
        });
    }

    pub fn new(seed: u64, width: u64, height: u64) -> Self {
        let mut gen = MapGen::new(seed, width, height);
        let map = gen.generate();
        let opengl = OpenGL::V4_5;
        MapGame {
            gl: opengl,
            mapdata: map,
        }
    }
    pub fn from_game_args(seed: u64, args: &Vec<&str>) -> Self {
        MapGame::new(seed, 200, 200)
    }
    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
    }
}


