pub fn get_generated_level(level: u32) -> crate::world::Level {
    
    match level {
	
	0 => {

	    let mut map_content = [[0;32];32];

	    // top wall
	    
	    map_content[8][8] = 1;
	    map_content[9][8] = 1;
	    map_content[10][8] = 1;
	    map_content[11][8] = 1;
	    map_content[12][8] = 1;
	    map_content[13][8] = 1;
	    map_content[14][8] = 1;


	    // bottom wall
	    
	    map_content[8][8] = 1;
	    map_content[8][9] = 1;
	    map_content[8][10] = 1;
	    map_content[8][11] = 1;
	    map_content[8][12] = 1;
	    map_content[8][13] = 1;
	    map_content[8][14] = 1;

	    // left wall

	    map_content[8][8+7] = 1;
	    map_content[9][8+7] = 1;
	    map_content[10][8+7] = 1;
	    map_content[11][8+7] = 1;
	    map_content[12][8+7] = 1;
	    map_content[13][8+7] = 1;
	    map_content[14][8+7] = 1;

	    // right wall

	    map_content[8+7][8] = 1;
	    map_content[8+7][9] = 1;
	    map_content[8+7][10] = 1;
	    map_content[8+7][11] = 1;
	    map_content[8+7][12] = 1;
	    map_content[8+7][13] = 1;
	    
	    map_content[8+7][14] = 1;
	    map_content[8+7][15] = 1;
	    for i in 9..15 {
		for j in 9..15 {
		map_content[i][j] = 2;
		}
	    }
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
