use std::collections::HashMap;
use crate::world::Level;

pub fn get_generated_level(level: u32) -> crate::world::Level {
    
    match level {
	
	0 => {

	    let mut map_content = [[1;32];32];

	    // top wall
	    
	    map_content[8][8] = 0;
	    map_content[9][8] = 0;
	    map_content[10][8] = 0;
	    map_content[11][8] = 0;
	    map_content[12][8] = 0;
	    map_content[13][8] = 0;
	    map_content[14][8] = 0;


	    // bottom wall
	    
	    map_content[8][8] = 0;
	    map_content[8][9] = 0;
	    map_content[8][10] = 0;
	    map_content[8][11] = 0;
	    map_content[8][12] = 0;
	    map_content[8][13] = 0;
	    map_content[8][14] = 0;

	    // left wall

	    map_content[8][8+7] = 0;
	    map_content[9][8+7] = 0;
	    map_content[10][8+7] = 0;
	    map_content[11][8+7] = 0;
	    map_content[12][8+7] = 0;
	    map_content[13][8+7] = 0;
	    map_content[14][8+7] = 0;

	    // right wall

	    map_content[8+7][8] = 0;
	    map_content[8+7][9] = 0;
	    map_content[8+7][10] = 0;
	    map_content[8+7][11] = 0;
	    map_content[8+7][12] = 0;
	    map_content[8+7][13] = 0;
	    
	    map_content[8+7][14] = 0;
	    map_content[8+7][15] = 0;


	    crate::world::Level {
	    
	    map_size: 32,
	    content: map_content
	}},
	1 => {

	    let map_content = [[1;32];32];
	    crate::world::Level {
	    
	    map_size: 32,
	    content: map_content
	}},
	_ => {

	    let map_content = [[1;32];32];
	    crate::world::Level {
	    
	    map_size: 32,
	    content: map_content
	}}
    }
}

pub fn get_station_map() -> crate::world::Level {

    let mut level = crate::world::Level {
	map_size: 32,
	content: [[1;32];32]
    };
    return level;
}
