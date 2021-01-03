use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use std::time::Duration;
use std::thread;
use std::io;
use world::Player;
mod world;
mod mapgenerator;
mod ui;

// "technical" constants
const SCREEN_HEIGHT: u32 = 600;
const SCREEN_WIDTH: u32 = 800;

const SCROLL_BOUNDARY_Y: f32 = 128.0;
const SCROLL_BOUNDARY_X: f32 = 256.0;
const TILE_SIZE: f32 = 32.0;
// gameplay constants
const PLAYER_SPEED: f32 = 4.0;
// controls



fn render(canvas: &mut WindowCanvas, player: &mut world::Player, map: &mut world::Level, camera: &mut world::Camera) {

    // per render things
    let bg_color = Color::RGB(0, 0, 0);
    let tile_color = Color::RGB(128,64,55);
    let player_color = Color::RGB(128,128,0);
    canvas.set_draw_color(bg_color);
    canvas.clear();
   
    
    for y in 0..map.map_size {
	for x in 0..map.map_size {
	    // render tile
	    if map.content[y as usize][x as usize] == 0 {
		canvas.set_draw_color(tile_color);
		canvas.fill_rect(Rect::new(y as i32 * TILE_SIZE as i32 - camera.x as i32, x as i32 * TILE_SIZE as i32 - camera.y as i32, TILE_SIZE as u32, TILE_SIZE as u32));
	    }
	    
	    if map.content[y as usize][x as usize] == 1 {
		canvas.set_draw_color(bg_color);
		canvas.fill_rect(Rect::new(y as i32 * TILE_SIZE as i32 - camera.x as i32, x as i32 * TILE_SIZE as i32 - camera.y as i32, TILE_SIZE as u32, TILE_SIZE as u32));
	    }
	}
    }
    // render player
    canvas.set_draw_color(player_color);
    canvas.fill_rect(Rect::new(player.x as i32 - camera.x as i32, player.y as i32 - camera.y as i32, 16,16));	
    canvas.present();
}


fn main_loop() -> Result<(), String> {
   //initialising windows and canvas 
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Descend", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    //initialising gameplay things

    let mut player = world::Player {
	
	x: 400.0,
	y: 300.0,
	speed_movement: 0.1,
	speed_rotation:0.1,

    };

    let mut camera = world::Camera {
	x: 0.0,
	y: 0.0
    };

    let mut map = mapgenerator::get_generated_level(0);

    let mut w = false;
    let mut a = false;
    let mut s = false;
    let mut d = false;

    // ui (command line ui is in its separate thread)
    
    thread::spawn(|| {
	loop {

	    
	    ui::draw_ui();
	    let mut line = String::new();
	    let input = std::io::stdin().read_line(&mut line).unwrap();
	    thread::sleep(Duration::from_millis(100));
	}
    }
    );

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
	    if map.content[((player.y - PLAYER_SPEED) % TILE_SIZE).floor() as usize ][((player.y - PLAYER_SPEED) % TILE_SIZE).floor() as usize] == 0 {
		player.y -= PLAYER_SPEED;
	    }
	    if player.y < camera.y + SCROLL_BOUNDARY_Y {
		camera.y -= PLAYER_SPEED;
	    }
	}
	
	if a == true {

	    player.x -= PLAYER_SPEED;
	    
	    if player.x < camera.x + SCROLL_BOUNDARY_X {
		camera.x -= PLAYER_SPEED;
	    }
	}
	
	if s == true {

	    player.y += PLAYER_SPEED;
	    
	    if player.y > camera.y + SCREEN_HEIGHT as f32 - SCROLL_BOUNDARY_Y {
		camera.y += PLAYER_SPEED;
	    }
	}
	
	if d == true {

	    player.x += PLAYER_SPEED;
	    
	    if player.x > camera.x + SCREEN_WIDTH as f32 - SCROLL_BOUNDARY_X {
		camera.x += PLAYER_SPEED;
	    }
	}
	// render
	
        render(&mut canvas, &mut player, &mut map, &mut camera);

	
	// sleep
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

fn main() {

main_loop();
}
