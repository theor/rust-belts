use specs::VecStorage;

pub struct DeltaTime(pub f32);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32
}