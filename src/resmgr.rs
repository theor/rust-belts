use std;
use ggez::Context;
use ggez::graphics::Image;
use find_folder;

pub struct ResMgr {
    assets: Vec<Sheet>,
}

pub struct Sheet {
    pub image: Image,
    pub stride:u8,
    pub size: (f32,f32),
    pub offset: (f32,f32),
}

impl ResMgr {
    pub fn new() -> Self {
        ResMgr {
            assets: Vec::new(),
        }
    }

pub fn load(&mut self, ctx: &mut Context, path: &'static str, stride:u8, size: (f32,f32), offset: (f32,f32)){
        let image = Image::new(
                ctx,
                &path
            ).unwrap();
        let size = (size.0 as f32 / image.width() as f32, size.1 / image.height() as f32);
        let offset = (offset.0 as f32 / image.width() as f32, offset.1 / image.height() as f32);
        self.assets.push(Sheet {
            image,
            stride,
            size,
            offset,
        });
    }

    pub fn try_get(&self, idx: usize) -> &Sheet {
        &self.assets[idx]
    }
}