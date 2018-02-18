use specs::{Fetch, ReadStorage, System as BaseSystem};
use piston_window::{Event, PistonWindow};
use resmgr::ResMgr;
use components::*;

pub struct System<'a>(pub &'a mut PistonWindow, pub &'a Event); //gfx_graphics::back_end::GfxGraphics<'_, gfx_device_gl::Resources, gfx_device_gl::command::CommandBuffer>);

impl<'a> BaseSystem<'a> for System<'a> {
    type SystemData = (
        Fetch<'a, Camera>,
        Fetch<'a, ResMgr>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderer>,
    );

    fn run(&mut self, (cam, res, pos, renderer): Self::SystemData) {
        self.0.draw_2d(self.1, |context, graphics| {
            use specs::Join;
            use piston_window::*;

            clear([1.0; 4], graphics);

            for (position, renderer) in (&pos, &renderer).join() {
                match renderer {
                    &Renderer::SpriteSheet(ref sprite) => {
                        let img = (*res).try_get(sprite.sheet).unwrap();
                        let pimage = Image::new().src_rect([4f64, 4f64, 32f64, 32f64]);
                        pimage.draw(
                            &img.image,
                            &context.draw_state,
                            context
                                .transform
                                .trans(cam.0 as f64, cam.1 as f64)
                                .trans(position.x as f64, position.y as f64),
                            graphics,
                        );
                    }
                    &Renderer::Shape(ref shape) => {
                        ellipse(
                            [1.0, 0.0, 0.0, 1.0],
                            [0.0, 0.0, shape.rect.0 as f64, shape.rect.1 as f64],
                            context
                                .transform
                                .trans(cam.0 as f64, cam.1 as f64)
                                .trans(position.x as f64, position.y as f64),
                            graphics,
                        );
                    }
                }

                // println!("Hello, {:?}", &position);
            }
            
            let mgr_read = mgr.read();
            let font = mgr_read.as_ref().unwrap().get_font();
            let mut glyphs_guard = font.write().unwrap();
            let mut glyphs = &mut *glyphs_guard;
            let transform = context.transform.trans(10.0, 100.0);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
                .draw("Hello world!", glyphs, &context.draw_state, transform, graphics)
                .unwrap();
        });
    }
}
