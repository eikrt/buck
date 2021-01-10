pub struct Entity {
   pub x: f32,
   pub y: f32,
   pub speed: f32, 
   pub speed_movement: f32,
    pub speed_rotation: f32,
    pub attacking: bool,
}
pub struct Camera {
    pub x: f32,
    pub y: f32
}
pub struct Level {

    pub map_size: i32,
    pub content: [[i32;32];32],
}
pub struct WorldMap {

    pub content: [[i32;16];16],
    pub ship_x: i32,
    pub ship_y: i32,
}
pub struct Drill {
    pub silver: i32,
    pub ammo: i32,
    pub nitro: i32,
	
    
    
}
pub struct Scenario {
    pub title: String,
    
}
