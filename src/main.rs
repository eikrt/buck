use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use std::time::Duration;

// "technical" constants

const SCREEN_HEIGHT: u32 = 600;
const SCREEN_WIDTH: u32 = 800;
const TILE_SIZE: f32 = 64.0;
const MAP_SIZE: usize = 32;

// gameplay constants
const PLAYER_SPEED: f32 = 4.0;
struct Player {
    x: f32,
    y: f32,
    speed_movement: f32,
    speed_rotation: f32,
}
// controls


fn render(canvas: &mut WindowCanvas, player: &mut Player) {

    // per render things
    let bg_color = Color::RGB(128, 128, 138);
    let tile_color = Color::RGB(128,128,128);
    let player_color = Color::RGB(0,0,0);
    canvas.set_draw_color(bg_color);
    canvas.clear();
    let mut map:[[i32;MAP_SIZE];MAP_SIZE] = [[1;MAP_SIZE];MAP_SIZE];
    map[1][1] = 1;
    map[1][2] = 0;
    map[1][3] = 0;
    map[1][4] = 0;
    map[1][5] = 0;
    map[1][6] = 0;
    map[1][7] = 0;
    map[2][7] = 0;
    map[4][7] = 0;
    map[5][7] = 0;
    map[5][7] = 0;
    map[6][7] = 0;
    map[5][7] = 0;
    
    for y in 0..MAP_SIZE {
	for x in 0..MAP_SIZE {
	    // render tile
	    canvas.set_draw_color(tile_color);
	    canvas.fill_rect(Rect::new(y as i32 * TILE_SIZE as i32, x as i32 * TILE_SIZE as i32, TILE_SIZE as u32, TILE_SIZE as u32));
	}
    }
    // render player
    canvas.set_draw_color(player_color);
    canvas.fill_rect(Rect::new(player.x as i32, player.y as i32, 32,32));	
    canvas.present();
}


fn main_loop() -> Result<(), String> {
   //initialising windows and canvas 
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Buck", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    //initialising gameplay things

    let mut player = Player {
	
	x: 2.0,
	y: 2.0,
	speed_movement: 0.1,
	speed_rotation:0.1,

    };

    
    // event handling


    let mut w = false;
    let mut a = false;
    let mut s = false;
    let mut d = false;

    
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
	// event handling
	for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
		// WASD
		Event::KeyDown{keycode: Some(Keycode::W), ..} => {
		    
		    w = true;
		    
		}
		Event::KeyDown{keycode: Some(Keycode::A), ..} => {
		    
		    
		    
		    a = true;
		}
		Event::KeyDown{keycode: Some(Keycode::S), ..} => {
		    
		    
		    s = true;
		}
		Event::KeyDown{keycode: Some(Keycode::D), ..} => {
		    
		    
		    d = true;
		}
		
		// WASD
		Event::KeyUp{keycode: Some(Keycode::W), ..} => {
		    
		    w = false;
		    
		}
		Event::KeyUp{keycode: Some(Keycode::A), ..} => {
		    
		    a = false;
		}
		Event::KeyUp{keycode: Some(Keycode::S), ..} => {
		    
		    
		    s = false;
		}
		Event::KeyUp{keycode: Some(Keycode::D), ..} => {
		    
		    
		    d = false;
		}

		
                _ => {}
            }
        }

	if w == true {
	    player.y -= PLAYER_SPEED;
	}
	
	if a == true {

	    player.x -= PLAYER_SPEED;
	}
	
	if s == true {

	    player.y += PLAYER_SPEED;
	}
	
	if d == true {

	    player.x += PLAYER_SPEED;
	}
	// render
        render(&mut canvas, &mut player);
	// sleep
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

fn main() {

main_loop();
}
