use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Map {
    pub tot_size: i32,
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<i8>,
}

impl Map {
    pub fn new(x: i32, y: i32) -> Self {
        let total = if (x * y) % 8 == 0 { (x * y) / 8 } else { (x * y) / 8 + 1 };
        Self {
            tot_size: x * y,
            width: x,
            height: y,
            tiles: vec![0; total as usize],
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = map_idx(self.width, x, y) / 8;

                if self.tiles[i] & (1 << ((x * y) % 8)) == 0 {
                    print!(".");
		} else {
                    print!("o");
		}
            }
            io::stdout().flush().unwrap();
            println!("");
	}
    }
}

pub fn map_idx(width: i32, x: i32, y: i32) -> usize {
    ((y * width) + x) as usize
}
