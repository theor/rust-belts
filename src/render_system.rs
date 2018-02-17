use specs::{Fetch, ReadStorage, System as BaseSystem};
use piston_window::{PistonWindow, Event, Image};
use resmgr::ResMgr;
use components::{Camera, Position, Sprite};

pub struct System<'a>(pub &'a mut PistonWindow, pub &'a Event);//gfx_graphics::back_end::GfxGraphics<'_, gfx_device_gl::Resources, gfx_device_gl::command::CommandBuffer>);

impl<'a> BaseSystem<'a> for System<'a> {
    type SystemData = (Fetch<'a, Camera>,
                       Fetch<'a, ResMgr>,
                       ReadStorage<'a, Position>,
                       ReadStorage<'a, Sprite>);

    fn run(&mut self, (cam, res, pos, sprite): Self::SystemData) {
        self.0.draw_2d(self.1, |context, graphics| {
            use specs::Join;
            use piston_window::*;

            clear([1.0; 4], graphics);

            for (position, sprite) in (&pos, &sprite).join() {
                let img = (*res).try_get(sprite.sheet).unwrap();
                let pimage = Image::new().src_rect([4f64,4f64,32f64,32f64]);

                // println!("Hello, {:?}", &position);
                pimage.draw(
                    &img.image,
                    &context.draw_state,
                    context.transform.trans(cam.0 as f64, cam.1 as f64).trans(position.x as f64, position.y as f64),
                    graphics,
                );
            }
                
        //         let transform = context.transform.trans(10.0, 100.0);
        //         text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
        //             .draw("Hello world!", &mut glyphs, &context.draw_state, transform, graphics);
            });

    }
}