extern crate find_folder;
extern crate piston_window;
extern crate specs;
extern crate graphics;
// extern crate sprite;
#[macro_use]
extern crate specs_derive;

use piston_window::*;
use specs::{DispatcherBuilder, World, };

mod components;
mod render_system;
mod move_system;
mod resmgr;

use components::{DeltaTime, Camera, Position, Sprite};

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Sprite>();

    world.add_resource(DeltaTime(0f32));
    world.add_resource(Camera(0f32, 50f32));

for i in 0..10 {
    world.create_entity()
        .with(Position{x:i as f32 * 32f32,y:0f32})
        .with(Sprite{sheet:"transport-belt.png", rect: (0u8,0u8)})
        .build();
    
}

    let mut dispatcher = DispatcherBuilder::new()
        .add(move_system::System, "move", &[])
    .build();

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut mgr = resmgr::ResMgr::new();
    {
        mgr.load(&mut window.factory, "transport-belt.png", 16, (40,40))
    }
    world.add_resource(mgr);

    // let ref font = assets.join("FiraSans-Regular.ttf");
    // let factory = window.factory.clone();
    // let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
    // let image   = Image::new().rect(graphics::rectangle::square(0.0, 0.0, 200.0));
    
    while let Some(event) = window.next() {

        if let Some(_r) = event.render_args() {
            use specs::RunNow;
            let mut render = render_system::System(&mut window, &event);
            render.run_now(&mut world.res);
        }

        //         let transform = context.transform.trans(10.0, 100.0);
        //         text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
        //             .draw("Hello world!", &mut glyphs, &context.draw_state, transform, graphics);

        if let Some(u) = event.update_args() {
            {
                let mut delta = world.write_resource::<DeltaTime>();
                *delta = DeltaTime(u.dt as f32);
            }
            dispatcher.dispatch(&mut world.res);
        }
    }
}
