use crate::prelude::*;
use crate::map::map_idx;

/**
 ** Ressources I used for making the algorithm approximatively O(1) in time complexity
 ** https://docs.google.com/document/d/19pHCD433tYsvAor0WObxa2qusAjKdx96kaf3z5I8XT8/edit
 ** https://stackoverflow.com/questions/20335427/most-efficient-algorithm-to-find-the-biggest-square-in-a-two-dimension-map
 ** https://stackoverflow.com/questions/1726632/dynamic-programming-largest-square-block
 ** c 1 2  r 
 **  |A|B| 1
 **  |C|D| 2
 **/

pub struct InfoMap {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<i32>,
    pub tile_sum: Vec<i32>,
    pub result: (usize, i32),
}

impl InfoMap {
    pub fn new(map: &MapBuilder) -> Self {
        let mut im = InfoMap {
            width: map.map.width,
            height: map.map.height,
            tiles: vec![0; (map.map.width * map.map.height) as usize],
            tile_sum: vec![0; (map.map.width * map.map.height) as usize],
            result: (0, 0)
        };
        im.fill_map(map);
        im
    }

    fn fill_map(&mut self, map: &MapBuilder) {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = map_idx(self.width, x, y);
                let k = x * y;
                let mut flag = 1;

		flag <<= k % 8;
		if map.map.tiles[i / 8] & flag != 0 {
                    self.tiles[i] = 1;
                    self.tile_sum[i] = 1;
		}
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let i = map_idx(self.width, x, y);
                let mut a = 0;
                let mut b = 0;
                let mut c = 0;
                if x > 0 {
                    c = map_idx(self.width, x - 1, y);
                }
                if y > 0 {
                    b = map_idx(self.width, x, y - 1);
                }
                if x > 0 && y > 0 {
                    a = map_idx(self.width, x - 1, y - 1);
                }
                self.tile_sum[i] = self.tile_sum[c] + self.tile_sum[b] - self.tile_sum[a] + self.tile_sum[i];
            }
        }
    }

    pub fn find_bsq(&mut self) -> (usize, i32) {
        let mut max_square = std::cmp::min(self.width, self.height);

        self.result = loop {
            if max_square <= 0 {
                break self.result;
            }
            for y in 0..self.height - max_square {
                if max_square <= 0 {
                    break;
                }
                for x in 0..self.width - max_square {
                    let a = {
                        if x - 1 < 0 || y - 1 < 0 {
                            0
                        } else {
                            self.tile_sum[map_idx(self.width, x - 1, y - 1)]
                        }
                    };
                    let b = {
                        if y - 1 < 0 {
                            0
                        } else {
                            self.tile_sum[map_idx(self.width, x + max_square - 1, y - 1)]
                        }
                    };
                    let c = {
                        if x - 1 < 0 {
                            0
                        } else {
                            self.tile_sum[map_idx(self.width, x - 1, y + max_square - 1)]
                        }
                    };
                    let d = self.tile_sum[map_idx(self.width, x + max_square - 1, y + max_square - 1)];
                    if d - c - b + a == 0 {
                        let i = map_idx(self.width, x, y);
                        self.result = (i, max_square);
                        max_square = 0;
                        break;
                    }
                }
            }
            max_square -= 1;
        };

        let square = self.result.1;
        let x_min = self.result.0 as i32 % self.width;
        let y_min = self.result.0 as i32 / self.width;
        let x_max = self.result.0 as i32 % self.width + square;
        let y_max = self.result.0 as i32 / self.width + square;

        for y in y_min..y_max {
            for x in x_min..x_max {
                let i = map_idx(self.width, x, y);
                self.tiles[i] = 2;
            }
        }
        return self.result;
    }

    pub fn print_algo(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = map_idx(self.width, x, y);
                print!("{:3}", self.tile_sum[i]);
            }
            io::stdout().flush().unwrap();
            println!("");
        }
    }

    pub fn print_bsq(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = map_idx(self.width, x, y);
                match self.tiles[i] {
                    0 => print!("."),
                    1 => print!("o"),
                    _ => print!("x"),
                }
            }
            io::stdout().flush().unwrap();
            println!("");
        }
    }
}
