use rand::Rng;



const empty_texts: &[&str] = &["You hear stones crumbling...", "The earth moves...", "You listen to beetles..."];
const scenarios: &[&str] = &["wounded_man", "empty_cavern", "drill_wreck", "ancient_ruins", "ancient_remains"];


pub fn draw_ui(state: &str, player: &crate::world::Entity) {
    //print!("{}[2J", 27 as char);
    println!("HP: ");
    for h in 0..player.hp {
	print!("x ", );
    }
    println!("");
    println!("Ammo: x x");
}
pub fn draw_descend(map: &mut crate::world::WorldMap) -> crate::world::Scenario{
    loop {
//	print!("{}[2J", 27 as char);
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

	
	if map.content[map.ship_y as usize][map.ship_x as usize] == 2 {

	    let mut rng = rand::thread_rng();
	    
	    if scenarios[rng.gen_range(0..scenarios.len())] == "wounded_man"{
		println!("You tear open a small cave where lies a man, clearly wounded.
1. Leave him be and keep drilling
2. Try to save him");
		let mut line2 = String::new();
	       let input2 = std::io::stdin().read_line(&mut line2).unwrap();
	       if line2.trim() == "1" {
		   
		    return crate::world::Scenario {
			title: "neutral".to_string()
		    };
		}
		
		else if line2.trim() == "2" {
		    
		    
		    return crate::world::Scenario {
			title: "intruder".to_string()
		    };
		}
	    }
	    
	    else if scenarios[rng.gen_range(0..scenarios.len())] == "empty_cavern"{
		
		println!("You face a dark and empty cavern.
1. Keep drilling
2. Enter the cave");
		
		let mut line2 = String::new();
		let input2 = std::io::stdin().read_line(&mut line2).unwrap();
		if line2.trim() == "1" {
		    
		    return crate::world::Scenario {
			title: "neutral".to_string()
		    };
		}
		
		if line2.trim() == "2" {
		    
		    return crate::world::Scenario {
			title: "board".to_string()
		    };
		}
	    }
	    
	    else if scenarios[rng.gen_range(0..scenarios.len())] == "drill_wreck"{
		
		println!("A wreck of a transport drill lies abandoned before you. Nothing has been here for a long time.
1. Keep drilling
2. Enter the drill");
		
		let mut line2 = String::new();
		let input2 = std::io::stdin().read_line(&mut line2).unwrap();
		if line2.trim() == "1" {
		    
		    return crate::world::Scenario {
			title: "neutral".to_string()
		    };
		}
		
		if line2.trim() == "2" {
		    
		    return crate::world::Scenario {
			title: "board".to_string()
		    };
		}
	    }
	    
	    else if scenarios[rng.gen_range(0..scenarios.len())] == "ancient_ruins"{
		
		println!("You tear open a small cave where lies a man, clearly wounded.
1. Leave him be and keep drilling
2. Try to save him");
		
		let mut line2 = String::new();
		let input2 = std::io::stdin().read_line(&mut line2).unwrap();
		if line2.trim() == "1" {
		    
		    return crate::world::Scenario {
			title: "neutral".to_string()
		    };
		}
		
		if line2.trim() == "2" {
		    return crate::world::Scenario {
			title: "intruder".to_string()
		    };
		}
	    }
	    
	    else if scenarios[rng.gen_range(0..scenarios.len())] == "ancient_remains"{
		
		println!("You stumble upon the remains of a giant insect. It seems prehistoric.
1. Keep drilling
2. Search the remains");
		
		let mut line2 = String::new();
		let input2 = std::io::stdin().read_line(&mut line2).unwrap();
		if line2.trim() == "1" {
		    return crate::world::Scenario {
			title: "neutral".to_string()
		    };
		}
		
		if line2.trim() == "2" {
		    return crate::world::Scenario {
			title: "loot".to_string()
		    };
		}
	    }
	}
	if map.content[map.ship_y as usize][map.ship_x as usize] == 0 {
	    let mut rng = rand::thread_rng();
	    
	    println!("{}", empty_texts[rng.gen_range(0..empty_texts.len())]);
	}
	map.content[map.ship_y as usize][map.ship_x as usize] = 1;
	
		    return crate::world::Scenario {
			title: "neutral".to_string()
		    };
    }
}
