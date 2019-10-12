
#[derive(Clone)]
pub struct Map {
    width: u64,
    height: u64,
    data: Vec<TileData>
}

#[derive(Clone, Copy)]
pub enum TileData {
    Valid {
        color: [f32; 4]
    },
    OutOfIndex
}

impl Map {

    pub fn width(&self) -> u64 {
        self.width
    }
    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn tile_data(&self, x: u64, y: u64) -> TileData {
        if x > self.width || y > self.height {
            return TileData::OutOfIndex;
        }
        self.data[(x * self.width + y) as usize]
    }
    pub fn new(data: Vec<TileData>, width: u64, height: u64) -> Self {
        assert_eq!(data.len() as u64 , width * height);
        Map {width: width, height: height,data: data}
    }
}

