
use super::map::{Map, TileData};
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;

pub struct MapGen {
    seed: u64,
    width: u64,
    height: u64
}
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const GREY: [f32; 4] = [0.75, 0.75, 0.75, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const SAND: [f32;4] = [0.761, 0.698,0.502,1.0];
        const DARKGREEN: [f32;4] = [0.0, 0.392,0.0,1.0];
impl MapGen {
    pub fn new(seed: u64, width: u64, height: u64) -> Self  {
        MapGen{seed: seed, width: width, height: height}
    }

    pub fn generate(&mut self) -> Map {
        let mut rng = super::super::get_rng(self.seed);
        let mut vec = Vec::with_capacity((self.width * self.height) as usize);
        let uni = Uniform::from(0..=4);
        for _ in 0 .. self.width {
            for _ in 0 .. self.height{
                vec.push(MapGen::get_tile_from_u64(uni.sample(&mut rng)))
            } 
        }
        Map::new(vec, self.width, self.height)
    }   

    fn get_tile_from_u64(val: u64) -> TileData {
        match val {
            0 => TileData::Valid{ color: BLUE},
            1 => TileData::Valid{ color: SAND},
            2 => TileData::Valid{ color: GREEN},
            3 => TileData::Valid{ color: DARKGREEN},
            4 => TileData::Valid{ color: GREY},
            _ => TileData::Valid{ color: BLUE}
        }
    }

}