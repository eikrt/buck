pub struct Player {
   pub x: f32,
   pub y: f32,
   pub speed_movement: f32,
   pub speed_rotation: f32,
}
pub struct Camera {
    pub x: f32,
    pub y: f32
}
pub struct Level {

    pub map_size: i32,
    pub content: [[i32;32];32],
}
