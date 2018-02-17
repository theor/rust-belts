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
#[component(VecStorage)]
pub struct GridItem {
    pub ix: u32,
    pub iy: u32,
    
    pub dx: u8,
    pub dy: u8,
}

impl GridItem {
    pub fn new(x: u32, y: u32) -> Self {
        GridItem {
            ix: x,
            iy: y,
            dx: 0u8,
            dy: 0u8,
        }
    }
}

#[derive(Component, Debug)]
#[component(DenseVecStorage)]
pub struct Belt {
}

#[derive(Component, Debug)]
#[component(DenseVecStorage)]
pub struct Item {
}

#[derive(Component, Debug)]
#[component(VecStorage)]
pub enum Renderer {
    SpriteSheet(Sprite),
    Shape(Shape),
}

impl Renderer {
    pub fn sprite(sheet: &'static str, rect: (u8,u8)) -> Self {
        Renderer::SpriteSheet(Sprite {
            sheet: sheet,
            rect: rect,
        })
    }
    pub fn shape(rect: (u8,u8)) -> Self {
        Renderer::Shape(Shape {
            rect: rect,
        })
    }
}

#[derive(Debug)]
pub struct Shape {
    pub rect: (u8,u8),
    // pub x: f32,
    // pub y: f32
}

#[derive(Debug)]
pub struct Sprite {
    pub sheet: &'static str,
    pub rect: (u8,u8),
    // pub x: f32,
    // pub y: f32
}