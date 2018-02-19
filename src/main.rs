#![feature(test)]

extern crate find_folder;
extern crate flame;
extern crate fps_counter;
extern crate graphics;
extern crate piston_window;
extern crate rayon;
extern crate specs;
// extern crate sprite;
#[macro_use]
extern crate specs_derive;

#[cfg(test)]
extern crate test;

mod components;
mod render_system;
mod move_system;
mod update_pos_system;
mod grid_system;
mod resmgr;
mod factory;
// mod quadtree;


use fps_counter::FPSCounter;
use piston_window::*;
use specs::{DispatcherBuilder, World, RunNow};
use components::{FPS, DeltaTime};

fn main() {
    let mut world = World::new();
    factory::init(&mut world);
    
     for j in 0..100 {
            for i in 0..100 {
                factory::belt(&mut world, i, j);
            }
            for i in 0..10 {
                factory::item(&mut world, i, j);
            }
        }
    // for i in 0..10 {
    //     factory::belt(&mut world, i, 0);
    // }
    // factory::item(&mut world, 0, 0);
    // factory::item(&mut world, 1, 0);
    // factory::item(&mut world, 0, 1);

    let mut dispatcher = DispatcherBuilder::new()
        .add(move_system::System::new(), "move", &[])
        .add(update_pos_system::System, "update_pos_system", &["move"])
        .build();

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut mgr = resmgr::ResMgr::new();
    {
        mgr.load(&mut window.factory, "transport-belt.png", 16, (40,40), (4,4));
        mgr.load(&mut window.factory, "copper-plate.png", 1, (32,32), (0,0));
    }
    let ref font = mgr.asset_path("FiraSans-Regular.ttf");
    world.add_resource(mgr);
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
    world.add_resource(FPS(0));
    // let image   = Image::new().rect(graphics::rectangle::square(0.0, 0.0, 200.0));
    
    grid_system::System::new().run_now(&mut world.res);
    let mut counter = FPSCounter::new();

    while let Some(event) = window.next() {

        if let Some(_r) = event.render_args() {

             {
                let mut delta = world.write_resource::<FPS>();
                *delta = FPS(counter.tick());
            }
            // let _guard = flame::start_guard("render");
            let mut render = render_system::System(&mut window, &event);
            let data = render.fetch(&mut world);
            render.run(data, &mut glyphs);
            // render.run_now(&mut world.res);

        }

        if let Some(u) = event.update_args() {
            {
                let mut delta = world.write_resource::<DeltaTime>();
                *delta = DeltaTime(u.dt as f32);
            }
            let _guard = flame::start_guard("update");
            dispatcher.dispatch(&mut world.res);
        }
    }

    use std::fs::File;
    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    pub fn bench(b: &mut Bencher) {
        let mut world = World::new();
        factory::init(&mut world);
        
        for j in 0..100 {
            for i in 0..100 {
                factory::belt(&mut world, i, j);
            }
            for i in 0..10 {
                factory::item(&mut world, i, j);
            }
        }
        
        grid_system::System::new().run_now(&mut world.res);

        let mut dispatcher = DispatcherBuilder::new()
            .add(move_system::System::new(), "move", &[])
            .add(update_pos_system::System, "update_pos_system", &["move"])
            .build();

        b.iter(||{
            use specs::RunNow;
            move_system::System::new().run_now(&mut world.res);
            // dispatcher.dispatch(&mut world.res);
        });
    }
}