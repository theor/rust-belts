use specs::prelude::*;
use specs::prelude::System as BaseSystem;
use piston_window::Glyphs;
use piston_window::{Event, PistonWindow};
use resmgr::ResMgr;
use components::*;

pub struct System<'a>(pub &'a mut PistonWindow, pub &'a Event); //gfx_graphics::back_end::GfxGraphics<'_, gfx_device_gl::Resources, gfx_device_gl::command::CommandBuffer>);


impl<'a> BaseSystem<'a> for System<'a> {
    type SystemData = (
        Fetch<'a, FPS>,
        Fetch<'a, Camera>,
        Fetch<'a, ResMgr>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderer>,
    );

    fn run(&mut self, _: Self::SystemData) {}
}

impl<'a> System<'a> {
    pub fn fetch(&self, world: &'a mut World) -> <Self as BaseSystem<'a>>::SystemData {
        <Self as BaseSystem<'a>>::SystemData::fetch(&mut world.res)
    }
    pub fn run(&mut self, (fps, cam, res, pos, renderer): <Self as BaseSystem<'a>>::SystemData, font:&mut Glyphs) {
        let w = { &mut (self.0) };
        w.draw_2d(self.1, |context, graphics| {
            use piston_window::*;

            clear([1.0; 4], graphics);

            // TODO: batch:
            // http://docs.piston.rs/mush/src/graphics/image.rs.html#99
            // http://docs.piston.rs/mush/src/opengl_graphics/back_end.rs.html#379
            use std::iter::Iterator;
            let mut iter = (&pos, &renderer).join();

            let img = (*res).try_get(0);
            let source_rectangle = [img.offset.0 as f64, img.offset.1 as f64, img.size.0 as f64, img.size.1 as f64];
            let uvs = &triangulation::rect_tri_list_uv(&img.image, source_rectangle);
             graphics.tri_list_uv(
                &context.draw_state,
                &[1.0; 4],
                &img.image,
                |f| {
                    triangulation::stream_polygon_tri_list(context
                        .transform
                        .trans(cam.0 as f64, cam.1 as f64),
                    || {
                        if let Some((position, renderer)) = iter.next() {
                            match renderer {
                                &Renderer::SpriteSheet(ref sprite) => {
                                    let img = (*res).try_get(sprite.sheet);
                                    let source_rectangle = [img.offset.0 as f64, img.offset.1 as f64, img.size.0 as f64, img.size.1 as f64];
                                    
                                    None
                                },
                                _ => None,
                            }
                        } else {
                            None
                        }
                    },
                    |xy| {f(xy, uvs)})
                }
            );

            if let Some((position, renderer)) = iter.next() {
                
                match renderer {
                    &Renderer::SpriteSheet(ref sprite) => {
                        let img = (*res).try_get(sprite.sheet);
                        let source_rectangle = [img.offset.0 as f64, img.offset.1 as f64, img.size.0 as f64, img.size.1 as f64];
                        let transform = context
                                .transform
                                .trans(cam.0 as f64, cam.1 as f64)
                                .trans(position.x as f64, position.y as f64);
           
                    },
                    _ => (),
                }
            // }


            // for (position, renderer) in (&pos, &renderer).join() {
            //     if position.x < 0.0 || position.y < 0.0 || position.x > 500.0 || position.y > 500.0 {
            //         continue;
            //     }
            //     match renderer {
            //         &Renderer::SpriteSheet(ref sprite) => {
            //             let img = (*res).try_get(sprite.sheet);
            //             let source_rectangle = [img.offset.0 as f64, img.offset.1 as f64, img.size.0 as f64, img.size.1 as f64];
            //             let rectangle = [0.0, 0.0, sprite.rect.0 as f64, sprite.rect.1 as f64];
                        
            //             let transform = context
            //                     .transform
            //                     .trans(cam.0 as f64, cam.1 as f64)
            //                     .trans(position.x as f64, position.y as f64);
            //             graphics.tri_list_uv(
            //                 &context.draw_state,
            //                 &[1.0; 4],
            //                 &img.image,
            //                 |f| f(
            //                     &triangulation::rect_tri_list_xy(transform, rectangle),
            //                     &triangulation::rect_tri_list_uv(&img.image, source_rectangle)
            //                 )

            //             );
            //             // let pimage = Image::new()
            //             //     .src_rect([img.offset.0 as f64, img.offset.1 as f64, img.size.0 as f64, img.size.1 as f64])
            //             //     .rect([0.0, 0.0, sprite.rect.0 as f64, sprite.rect.1 as f64]);
            //             // pimage.draw(
            //             //     &img.image,
            //             //     &context.draw_state,
            //             //     context
            //             //         .transform
            //             //         .trans(cam.0 as f64, cam.1 as f64)
            //             //         .trans(position.x as f64, position.y as f64),
            //             //     graphics,
            //             // );
            //         }
            //         &Renderer::Shape(ref shape) => {
            // //             // ellipse(
            // //             //     [1.0, 0.0, 0.0, 1.0],
            // //             //     [0.0, 0.0, shape.rect.0 as f64, shape.rect.1 as f64],
            // //             //     context
            // //             //         .transform
            // //             //         .trans(cam.0 as f64, cam.1 as f64)
            // //             //         .trans(position.x as f64, position.y as f64),
            // //             //     graphics,
            // //             // );
            //         }
            //     }

            // // //     // println!("Hello, {:?}", &position);
            // }
            
           
            let fps = fps.0;
            let transform = context.transform.trans(10.0, 100.0);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
                .draw(&format!("{}fps", fps), font, &context.draw_state, transform, graphics)
                .unwrap();
        });
    }
}
