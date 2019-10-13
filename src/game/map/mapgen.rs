extern crate rand_distr;
extern crate voronoi;

use super::map::{Map, TileData};
use rand::distributions::{Distribution, Uniform};
use voronoi::{Point, voronoi, DCEL, make_line_segments};
use rand::{Rng};
use rand::rngs::{StdRng};

pub enum GenState{
    Start,
    Points(Vec<Point>),
    Voronoi(DCEL, Vec<Point>),

    End
}

pub struct MapGen {
    rng: StdRng,
    width: u64,
    height: u64,
    state: GenState,
    seed: u64
}


const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const GREY: [f32; 4] = [0.75, 0.75, 0.75, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const SAND: [f32; 4] = [0.761, 0.698, 0.502, 1.0];
const DARKGREEN: [f32; 4] = [0.0, 0.392, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
impl MapGen {
    pub fn new(seed: u64, width: u64, height: u64) -> Self {
        MapGen {
            rng: super::super::get_rng(seed),
            width: width,
            height: height,
            state: GenState::Start,
            seed: seed
        }
    }

    // pub fn generate(&mut self) -> Map {
    //     let mut rng = super::super::get_rng(self.seed);
    //     let mut vec = Vec::with_capacity((self.width * self.height) as usize);
    //     let uni = Uniform::from(0..=4);
    //     for _ in 0..self.width {
    //         for _ in 0..self.height {
    //             vec.push(get_tile_from_u64(uni.sample(&mut rng)))
    //         }
    //     }
    //     Map::new(vec, self.width, self.height)
    // }

    pub fn next(&mut self)-> Box<dyn Fn(graphics::Context, &mut opengl_graphics::GlGraphics)> {
        super::super::super::println_with_color( "next",console::Color::Red);
        match  &self.state {
            GenState::Start => {
                let points = gen_points(self.width, self.height,&mut self.rng);
                self.state = GenState::Points(points.clone());
                Box::new(move |c, gl| {
                    use graphics::*;
                    clear(WHITE, gl);
                    for p in points.clone() {
                        let trans = c.transform.trans(p.x.into() ,p.y.into());
                        let square = rectangle::centered_square(0.0, 0.0, 1.0);
                        graphics::rectangle(BLACK,square,trans, gl);
                    }
                })
            },
            GenState::Points(points) => {
                let copy = points.clone();
                let relaxed = voronoi::lloyd_relaxation(copy, self.width.max(self.height) as f64);
                let dcel = voronoi(relaxed.clone(), self.width.max(self.height) as f64);
                let lines =  make_line_segments(&dcel);
                let draw = relaxed.clone();
                self.state = GenState::Voronoi(dcel, relaxed);
                Box::new(move |c, gl|{
                    use graphics::*;
                    clear(WHITE, gl);
                    for p in draw.clone() {
                        let trans = c.transform.trans(p.x.into() ,p.y.into());
                        let square = rectangle::centered_square(0.0, 0.0, 1.0);
                        graphics::rectangle(BLACK,square,trans, gl);
                    }
                    for l in lines.clone() {
                        line_from_to(BLUE, 1.0, [l[0].x.into(),l[0].y.into()], [l[1].x.into(),l[1].y.into()], c.transform, gl)
                    }
                })
            },
            GenState::Voronoi(ref dcel, ref points) => {
                let polygons = voronoi::make_polygons(&dcel);
                let colors : Vec<[f32;4]> = polygons.iter().map(|_| { get_random_color()}).collect();
                self.state = GenState::End;
                Box::new(move |c, gl|{
                        use graphics::*;
                        clear(WHITE, gl);
                        let mut i = 0;
                    for p in polygons.iter(){
                        let mut vec = Vec::new();
                        for po in p.iter(){
                            vec.push([po.x.into(),po.y.into()]);
                        }
                        polygon(colors[i], &vec , c.transform, gl);
                        i += 1;
                    }
                })
            },
            _ => Box::new(move |c,gl|{
                
            })
        }
        
    }

    pub fn restart(&mut self) {
        self.state = GenState::Start;
        self.rng = super::super::get_rng(self.seed)
    }
}


fn get_random_color() -> [f32;4]{
    let mut rng = rand::thread_rng();
    let val = rng.gen_range(0,4);
    match val {
        0 => GREEN,
        1 => BLACK,
        2 => BLUE,
        3 => GREY,
        _ => WHITE
    }
}




fn gen_points(width: u64, height: u64, rng: &mut impl Rng) -> Vec<Point> {
    let uniform_w = Uniform::new(0.0, width as f64);
    let uniform_h = Uniform::new(0.0, height as f64);

    let count_x = rng.gen_range(5, width/10);
    let count_y = rng.gen_range(5,height/10 );
    let cap = (count_x * count_y) as usize;
    let mut vec = Vec::with_capacity(cap);
    let x_coords : Vec<f64> = rng.sample_iter(uniform_w).take(cap).collect();
    let y_coords : Vec<f64> = rng.sample_iter(uniform_h).take(cap).collect();
    for i in 0 .. cap {
        vec.push(Point::new(x_coords[i], y_coords[i]));
    }
    return vec;
}


fn get_tile_from_u64(val: u64) -> TileData {
    match val {
        0 => TileData::Valid { color: BLUE },
        1 => TileData::Valid { color: SAND },
        2 => TileData::Valid { color: GREEN },
        3 => TileData::Valid { color: DARKGREEN },
        4 => TileData::Valid { color: GREY },
        _ => TileData::Valid { color: BLUE },
    }}
