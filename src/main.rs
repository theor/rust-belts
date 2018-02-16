extern crate find_folder;
extern crate piston_window;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use piston_window::*;
use specs::{DispatcherBuilder, World, };

mod components;
mod render_system;
mod move_system;

use components::{DeltaTime, Position};

fn main() {
    let mut world = World::new();
    world.register::<Position>();

    world.add_resource(DeltaTime(16.6f64));

    world.create_entity()
        .with(Position{x:0f32,y:0f32})
        .build();

    world.create_entity()
        .with(Position{x:500f32,y:200f32})
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .add(move_system::System, "move", &[])
    .build();

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    println!("{:?}", assets);
    // let ref font = assets.join("FiraSans-Regular.ttf");
    // let factory = window.factory.clone();
    // let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

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
                *delta = DeltaTime(u.dt);
            }
            dispatcher.dispatch(&mut world.res);
        }
    }
}
