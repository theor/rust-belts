use specs;
use specs::{VecStorage, ReadStorage};

use piston_window::{PistonWindow, Event};

use components::Position;
// use piston::graphics;

pub struct System<'a>(pub &'a mut PistonWindow, pub &'a Event);//gfx_graphics::back_end::GfxGraphics<'_, gfx_device_gl::Resources, gfx_device_gl::command::CommandBuffer>);

impl<'a> specs::System<'a> for System<'a> {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, data: Self::SystemData) {
        self.0.draw_2d(self.1, |context, graphics| {
            use specs::Join;
            use piston_window::*;

            clear([1.0; 4], graphics);

            for position in data.join() {
                // println!("Hello, {:?}", &position);
                rectangle(
                    [1.0, 0.0, 0.0, 1.0], // red
                    [position.x as f64, position.y as f64, 100.0, 100.0],
                    context.transform,
                    graphics,
                );
            }

                
        //         let transform = context.transform.trans(10.0, 100.0);
        //         text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
        //             .draw("Hello world!", &mut glyphs, &context.draw_state, transform, graphics);
            });

    }
}