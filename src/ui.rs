const empty_texts: &[&str] = &["You hear stones crumbling..."];


pub fn draw_ui(state: &str) {
    print!("{}[2J", 27 as char);
    println!("HP: x x x");
    println!("Ammo: x x");
}
pub fn draw_descend(map: &mut crate::world::WorldMap) -> &str{
    loop {
	print!("{}[2J", 27 as char);
	println!("Drill straight down, left or right? (d l r)");
	for i in 0..map.content.len() {
	    for j in 0..map.content.len() {
		print!("{}", map.content[i][j]);
	    }
	    println!("{}", "");
	}
	
	let mut line = String::new();
	let input = std::io::stdin().read_line(&mut line).unwrap();

	//refresh map

	
	

	
	map.content[map.ship_y as usize][map.ship_x as usize] = 0;
	
	if line.trim() == "d" {
	    
	    map.ship_y += 1;
	}
	
	if line.trim() == "l" {

	    
	    map.ship_x -= 1;
	    map.ship_y += 1;
	}
	
	if line.trim() == "r" {

	    map.ship_x += 1;
	    map.ship_y += 1;
	}

	//refresh map
	
	if map.content[map.ship_y as usize][map.ship_x as usize] == 2 {
	   
	    
	}
	if map.content[map.ship_y as usize][map.ship_x as usize] == 0 {
	    println!("{}", empty_texts[0]);
	}
	map.content[map.ship_y as usize][map.ship_x as usize] = 1;
	return "";
    }
}
