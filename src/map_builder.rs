use rand::Rng;

use crate::prelude::*;
use crate::map::map_idx;

#[derive(Debug, Clone)]
pub struct MapBuilder {
    pub map: Map,
    pub density: i32,
}

impl MapBuilder {
    pub fn new(x: i32, y: i32, density: i32) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(x, y),
            density: density,
        };

        mb.add_obstacles();
        mb
    }

    pub fn print(&self) {
        self.map.print();
    }

    pub fn add_obstacles(&mut self) {
        let mut rng = rand::thread_rng();

        println!("{}", self.density);
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if rng.gen_range(0..100) < self.density {
                    let i = map_idx(self.map.width, x, y) / 8;
		    let k = x * y;

                    self.map.tiles[i] |= 1 << (k % 8);
                }
            }
        }
    }
}
