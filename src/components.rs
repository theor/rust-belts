use specs::prelude::*;
use ntree::Region;

pub struct DeltaTime(pub f32);
pub struct Camera(pub f32, pub f32);

use ntree::{NTree};
pub type GridTree = NTree<GridRegion, RegionItem>;
pub struct Grid(pub GridTree);
use std::sync::atomic::AtomicUsize;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new() -> Self {
        Position { x: 0.0, y: 0.0 }
    }

    pub fn at(x: f32, y: f32) -> Self {
        Position { x: x, y: y }
    }
}

#[derive(Debug, Clone)]
pub struct GridRegion(pub u32, pub u32, pub u32, pub u32);
#[derive(Debug, Clone, PartialEq)]
pub struct RegionItem {
    pub ix: u32,
    pub iy: u32,
    pub e: Entity,
}

impl RegionItem {
    pub fn new(ix: u32, iy: u32, e: Entity) -> Self {
        RegionItem { ix, iy, e }
    }
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

    pub fn move_delta(&mut self, x: i16, y: i16) -> bool {
        let mut res = false;
        if x >= 0 {
            let new_dx = self.dx as u16 + x as u16;
            self.dx = (new_dx % 256) as u8;
            let ax = new_dx as u32 / 256;
            self.ix += ax;
            res |= ax != 0;
        } else {
            let new_dx = self.dx as i32 + x as i32;
            if new_dx < 0 { 
                self.dx = ((256 - (-new_dx as u32)) % 256) as u8;
                let ax = ((256-new_dx) as u32) / 256;
                self.ix -= ax;
                res |= ax != 0;
            } else {
                self.dx = (new_dx as u32 % 256) as u8;
                let ax = new_dx as u32 / 256;
                self.ix += ax;
                res |= ax != 0;
            }
        }
        if y >= 0 {
            let new_dy = self.dy as u32 + y as u32;
            self.dy = (new_dy % 256) as u8;
            let ay = new_dy as u32 / 256;
            self.iy += ay;
            res |= ay != 0;
        } else {
            let new_dy = self.dy as i32 + y as i32;
            if new_dy < 0 { 
                self.dy = ((256 - (-new_dy as u32)) % 256) as u8;
                let ay = ((256-new_dy) as u32) / 256;
                self.iy -= ay;
                res |= ay != 0;
            } else {
                self.dy = (new_dy as u32 % 256) as u8;
                let ay = new_dy as u32 / 256;
                self.iy += ay;
                res |= ay != 0;
            }
        }
        res
    }

    pub fn compute_position(&self) -> (f32, f32) {
        (
            (self.ix as f32 + self.dx as f32 / 255.0) * 32f32 + 16.0,
            (self.iy as f32 + self.dy as f32 / 255.0) * 32f32 + 16.0,
        )
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
pub struct Belt{
    pub direction: Direction,
    pub items: Vec<AtomicUsize>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    Up,Down,Left,Right,
}

impl Belt {
    pub fn new(direction:Direction) -> Self {
        use std::iter;
        Belt{ direction, items: iter::repeat(0).take(12).map (|_| AtomicUsize::new(0)).collect(), }
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Item {}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub enum Renderer {
    SpriteSheet(Sprite),
    Shape(Shape),
}

impl Renderer {
    pub fn sprite(sheet: usize, rect: (u8, u8), scale: (f32, f32), flip: Flip) -> Self {
        Renderer::SpriteSheet(Sprite {
            sheet,
            rect,
            scale,
            flip,
        })
    }
    pub fn shape(rect: (u8, u8)) -> Self {
        Renderer::Shape(Shape { rect: rect })
    }
}

#[derive(Debug)]
pub struct Shape {
    pub rect: (u8, u8),
    // pub x: f32,
    // pub y: f32
}

#[derive(Debug, Clone, PartialEq)]
pub enum Flip { None, Horizontal, Vertical, Both, }
#[derive(Debug)]
pub struct Sprite {
    pub sheet: usize,
    pub rect: (u8, u8),
    pub scale: (f32, f32),
    pub flip: Flip,
    // pub x: f32,
    // pub y: f32
}
