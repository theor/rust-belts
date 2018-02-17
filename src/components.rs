use specs::{DenseVecStorage,VecStorage};

pub struct DeltaTime(pub f32);
pub struct Camera(pub f32, pub f32);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

#[derive(Component, Debug)]
#[component(DenseVecStorage)]
pub struct Belt {}

#[derive(Component, Debug)]
#[component(DenseVecStorage)]
pub struct Item {}

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Sprite {
    pub sheet: &'static str,
    pub rect: (u8,u8),
    // pub x: f32,
    // pub y: f32
}