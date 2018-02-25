use specs::prelude::*;
use specs::world::Index;
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;
use ntree::Region;

pub struct DeltaTime(pub f32);
pub struct Camera(pub f32, pub f32);
pub struct FPS(pub usize);
pub struct Grid(pub Arc<RwLock<HashMap<(u32,u32), Vec<Index>>>>);

impl Grid {
    pub fn new() -> Self {
        Grid(Arc::new(RwLock::new(HashMap::new())))
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

impl Position {
    pub fn new() -> Self {
        Position {
            x: 0.0,
            y: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GridRegion(pub u32, pub u32, pub u32, pub u32);
#[derive(Debug, Clone)]
pub struct RegionItem {
    pub ix: u32,
    pub iy: u32,
    pub e: Entity,
}

impl RegionItem {
    pub fn new(ix: u32, iy: u32, e: Entity) -> Self { RegionItem {ix, iy, e} }
}

impl Region<RegionItem> for GridRegion {
 /// Does this region contain this point?
    fn contains(&self, i: &RegionItem) -> bool {
        i.ix >= self.0 && i.iy >= self.1 && i.ix < self.2 && i.iy < self.3
    }

    /// Split this region, returning a Vec of sub-regions.
    ///
    /// Invariants:
    ///   - The sub-regions must NOT overlap.
    ///   - All points in self must be contained within one and only one sub-region.
    fn split(&self) -> Vec<Self> {
        let sx = (self.2 - self.0) / 2;
        let sy = (self.3 - self.1) / 2;
        let mx = self.0 + sx;
        let my = self.1 + sy;
        vec![
            GridRegion(self.0, self.1, mx, my),
            GridRegion(mx, self.1, self.2, my),
            GridRegion(self.0, my, mx, self.3),
            GridRegion(mx, my, self.2, self.3),
        ]
    }

    /// Does this region overlap with this other region?
    fn overlaps(&self, other: &Self) -> bool {
        let ox = (self.0 >= other.0 && self.0 < other.2) || (other.0 >= self.0 && other.0 < self.2);
        let oy = (self.1 >= other.1 && self.1 < other.3) || (other.1 >= self.1 && other.1 < self.3);
        ox && oy
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
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
    
    pub fn new_subpos(x: u32, y: u32, dx: u8, dy: u8) -> Self {
        GridItem {
            ix: x,
            iy: y,
            dx: dx,
            dy: dy,
        }
    }

    pub fn move_delta(&mut self, x: i16, y: i16) {
        if x >= 0 {
            let new_dx = self.dx as u32 + x as u32;
            self.dx = (new_dx % 256) as u8;
            self.ix += new_dx / 256;
        } else {

        }
        if y >= 0 {
            let new_dy = self.dy as u32 + y as u32;
            self.dy = (new_dy % 256) as u8;
            self.iy += new_dy / 256;
        } else {

        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct GridVelocity {
    pub dx: i16,
    pub dy: i16,
}

impl GridVelocity {
    pub fn new() -> Self {
        GridVelocity { dx: 0, dy: 0 }
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Belt {
    pub items: Vec<Entity>,
}

impl Belt {
    pub fn new() -> Self { Belt { items: Vec::with_capacity(8) } }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Item {
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub enum Renderer {
    SpriteSheet(Sprite),
    Shape(Shape),
}

impl Renderer {
    pub fn sprite(sheet: usize, rect: (u8,u8)) -> Self {
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
    pub sheet: usize,
    pub rect: (u8,u8),
    // pub x: f32,
    // pub y: f32
}