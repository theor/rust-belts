use std;
use std::collections::HashMap;
use find_folder;
use piston_window::{G2dTexture, GfxFactory, PistonWindow, Texture, TextureSettings, Flip};
use piston_window;

pub struct ResMgr {
    assets_path: std::path::PathBuf,
    assets: HashMap<&'static str, Sheet>,
}

pub struct Sheet {
    pub image: G2dTexture,
    pub stride:u8,
    pub size: (u8,u8),
}

impl ResMgr {
    pub fn new() -> Self {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        println!("{:?}", assets);
        ResMgr {
            assets_path: assets,
            assets: HashMap::new(),
        }
    }
    pub fn load(&mut self, factory: &mut GfxFactory, path: &'static str, stride:u8, size: (u8,u8)){
        let belt_sheet = self.assets_path.join(path);
        let belt_image: G2dTexture = Texture::from_path(
                factory,
                &belt_sheet,
                Flip::None,
                &TextureSettings::new()
            ).unwrap();
        self.assets.insert(path, Sheet {image: belt_image, stride: stride, size: size});
    }

    pub fn try_get(&self, name: &'static str) -> Option<&Sheet> {
        self.assets.get(name)
    }
}