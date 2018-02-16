use specs::{VecStorage, ReadStorage};

pub struct DeltaTime(pub f64);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32
}