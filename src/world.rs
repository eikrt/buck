use rand::Rng;
pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub speed: f32, 
    pub speed_movement: f32,
    pub speed_rotation: f32,
    pub hp: i32,
    pub ammo: i32,
    pub attacking: bool,
    pub alive: bool
}
impl Entity {
    pub fn reset (&mut self) {
	self.x = 64.0;
	self.y = 64.0;
    }
}
pub struct AttackBox {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub angle: f32
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
impl Drill {
    pub fn find_loot(&mut self) {
	
	let mut rng = rand::thread_rng();
	    println!("Expedition complete");
	let silver = rng.gen_range(0..10);
	let ammo = rng.gen_range(0..4);
	let nitro = rng.gen_range(0..2);
	println!("You got {} amount of silver
{} amount of ammo
{} amount of nitro!", silver, ammo, nitro);
	    
	self.silver += silver;
	self.ammo += ammo;
	self.nitro += nitro;
    }
}
pub struct Scenario {
    pub title: String,
    
}
