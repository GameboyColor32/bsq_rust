pub mod map;
pub mod info_map;
pub mod map_builder;

mod prelude {
    pub use std::io;
    pub use std::io::Write;
    pub use crate::map::Map;
    pub use crate::map_builder::MapBuilder;
    pub use crate::info_map::InfoMap;
}

use prelude::*;

fn main() {
    // Step 1: Create a x * y map with random obstacles
    // This struct will generate a map which is actually a bitfield.
    // So its size will be: x * y / 8 + ((x * y % 8) ? 1 : 0)
    // instead of: x * y
    // The generation of the map and the BSQ algorithm are separated, this is why I
    // didn't generate the map inside the InfoMap.
    let map: MapBuilder = MapBuilder::new(100, 50, 10);

    map.print();
    println!("");

    // Step 2: Create a map of int
    // Since the MapBuilder struct contains a bitmask, I have to convert it
    // to a new map, and then performs the algorithm on a copy of the map
    let mut info_map: InfoMap = InfoMap::new(&map);

    // The print algo was used to check if the map filled with digits was accurate
    //info_map.print_algo();

    // Step 3: Find BSQ
    // The algorithm takes O(n * m) space & time complexity which
    // is the most optimal solution for this problem
    // @todo: implement the binary search algorithm and compare both performances
    info_map.find_bsq();
    info_map.print_bsq();
}
