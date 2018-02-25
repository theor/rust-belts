use specs::prelude::*;
use specs::prelude::System as BaseSystem;
use resmgr::ResMgr;
use components::*;

use ggez::*;
use ggez::graphics::{DrawMode, DrawParam, Point2, Rect};

pub struct System<'a>(pub &'a mut Context); //gfx_graphics::back_end::GfxGraphics<'_, gfx_device_gl::Resources, gfx_device_gl::command::CommandBuffer>);

impl<'a> BaseSystem<'a> for System<'a> {
    type SystemData = (
        Fetch<'a, Camera>,
        Fetch<'a, ResMgr>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderer>,
    );

    fn run(&mut self, (cam, res, pos, renderer): Self::SystemData) {
        for (position, renderer) in (&pos, &renderer).join() {
            if position.x < 0.0 || position.y < 0.0 || position.x > 500.0 || position.y > 500.0 {
                continue;
            }
            let ctx = &mut self.0;
            match renderer {
                &Renderer::SpriteSheet(ref sprite) => {
                    // graphics::circle(self.0,
                    //  DrawMode::Fill,
                    //  Point2::new(position.x, position.y),
                    //  20.0,
                    //  2.0).unwrap();
                    let img = (*res).try_get(sprite.sheet);
                    let source_rectangle = Rect::new(
                        img.offset.0 as f32,
                        img.offset.1 as f32,
                        img.size.0 as f32,
                        img.size.1 as f32,
                    );
                    let rectangle = Rect::new(0.0, 0.0, sprite.rect.0 as f32, sprite.rect.1 as f32);
                    // println!("src {:?} dst {:?} pos {:?}", source_rectangle, rectangle, position);
                    let dest = Point2::new(position.x, position.y);
                    // let dest = Point2::new(0.0,0.0);
                    graphics::draw_ex(
                        ctx,
                        &img.image,
                        DrawParam {
                            src: source_rectangle,
                            dest: dest,
                            ..Default::default()
                        },
                    ).unwrap();
                    // let transform = context
                    //         .transform
                    //         .trans(cam.0 as f64, cam.1 as f64)
                    //         .trans(position.x as f64, position.y as f64);
                    // graphics.tri_list_uv(
                    //     &context.draw_state,
                    //     &[1.0; 4],
                    //     &img.image,
                    //     |f| f(
                    //         &triangulation::rect_tri_list_xy(transform, rectangle),
                    //         &triangulation::rect_tri_list_uv(&img.image, source_rectangle)
                    //     )

                    // );
                }
                &Renderer::Shape(ref shape) => {
                    //             // ellipse(
                    //             //     [1.0, 0.0, 0.0, 1.0],
                    //             //     [0.0, 0.0, shape.rect.0 as f64, shape.rect.1 as f64],
                    //             //     context
                    //             //         .transform
                    //             //         .trans(cam.0 as f64, cam.1 as f64)
                    //             //         .trans(position.x as f64, position.y as f64),
                    //             //     graphics,
                    //             // );
                }
            }

            // //     // println!("Hello, {:?}", &position);
        }
    }
}

// impl<'a> System<'a> {
//     pub fn fetch(&self, world: &'a mut World) -> <Self as BaseSystem<'a>>::SystemData {
//         <Self as BaseSystem<'a>>::SystemData::fetch(&mut world.res)
//     }
//     pub fn run(&mut self, (fps, cam, res, pos, renderer): <Self as BaseSystem<'a>>::SystemData, font:&mut Glyphs) {
//         let w = { &mut (self.0) };
//         w.draw_2d(self.1, |context, graphics| {
//             use piston_window::*;

//             clear([0.5,0.5,0.5, 1.0], graphics);

//             // TODO: batch:
//             // http://docs.piston.rs/mush/src/graphics/image.rs.html#99
//             // http://docs.piston.rs/mush/src/opengl_graphics/back_end.rs.html#379
//             use std::iter::Iterator;
//             let mut iter = (&pos, &renderer).join();
//             let mut i = 6;
//             let mut x = None;

//             let img = (*res).try_get(0);
//             let source_rectangle = [img.offset.0 as f64, img.offset.1 as f64, img.size.0 as f64, img.size.1 as f64];
//             let uvs = &triangulation::rect_tri_list_uv(&img.image, source_rectangle);
//              graphics.tri_list_uv(
//                 &context.draw_state,
//                 &[1.0; 4],
//                 &img.image,
//                 |f| {
//                     triangulation::stream_polygon_tri_list(context
//                         .transform
//                         .trans(cam.0 as f64, cam.1 as f64),
//                     || {
//                         if i == 6 {
//                             x = iter.next();
//                             i = 0;
//                         }
//                         if let Some((position, renderer)) = x {
//                             match renderer {
//                                 &Renderer::SpriteSheet(ref sprite) => {
//                                     let (x, y, w, h) = (0.0, 0.0, sprite.rect.0 as f64, sprite.rect.1 as f64);
//                                     let (x2, y2) = (x + w, y + h);
//                                     i += 1;
//                                      let m = math::identity()
//                                          .trans(position.x as f64, position.y as f64);
//                                     use triangulation::{tx,ty};

//                                     let res = match i {
//                                         1 => Some([tx(m,x,y) as f64, ty(m,x,y) as f64]),
//                                         2 => Some([tx(m,x2,y) as f64, ty(m,x2,y) as f64]),
//                                         3 => Some([tx(m,x,y2) as f64, ty(m,x,y2) as f64]),
//                                         4 => Some([tx(m,x2,y) as f64, ty(m,x2,y) as f64]),
//                                         5 => Some([tx(m,x2,y2) as f64, ty(m,x2,y2) as f64]),
//                                         6 => Some([tx(m,x,y2) as f64, ty(m,x,y2) as f64]),
//                                         _ => { println!("i none {}", i); None },
//                                     };
//                                     println!("res {:?} i {}", res, i);
//                                     res
//                                 },
//                                 _ => { println!("shape none"); None },
//                             }
//                         } else {
//                             println!("end none");
//                             None
//                         }
//                     },
//                     |xy| {
//                         use std::slice::Iter;
//                         let uvs: Vec<[f32;2]> = uvs.iter().cloned().cycle().take(xy.len()).collect::<Vec<[f32;2]>>();
//                         println!("XYs {}", xy.len());
//                         println!("{:?}", xy);
//                         println!("{:?}", uvs);
//                         f(xy, &uvs)
//                     })
//                 }
//             );

//             // for (position, renderer) in (&pos, &renderer).join() {
//             //     if position.x < 0.0 || position.y < 0.0 || position.x > 500.0 || position.y > 500.0 {
//             //         continue;
//             //     }
//             //     match renderer {
//             //         &Renderer::SpriteSheet(ref sprite) => {
//             //             let img = (*res).try_get(sprite.sheet);
//             //             let source_rectangle = [img.offset.0 as f64, img.offset.1 as f64, img.size.0 as f64, img.size.1 as f64];
//             //             let rectangle = [0.0, 0.0, sprite.rect.0 as f64, sprite.rect.1 as f64];

//             //             let transform = context
//             //                     .transform
//             //                     .trans(cam.0 as f64, cam.1 as f64)
//             //                     .trans(position.x as f64, position.y as f64);
//             //             graphics.tri_list_uv(
//             //                 &context.draw_state,
//             //                 &[1.0; 4],
//             //                 &img.image,
//             //                 |f| f(
//             //                     &triangulation::rect_tri_list_xy(transform, rectangle),
//             //                     &triangulation::rect_tri_list_uv(&img.image, source_rectangle)
//             //                 )

//             //             );
//             //             // let pimage = Image::new()
//             //             //     .src_rect([img.offset.0 as f64, img.offset.1 as f64, img.size.0 as f64, img.size.1 as f64])
//             //             //     .rect([0.0, 0.0, sprite.rect.0 as f64, sprite.rect.1 as f64]);
//             //             // pimage.draw(
//             //             //     &img.image,
//             //             //     &context.draw_state,
//             //             //     context
//             //             //         .transform
//             //             //         .trans(cam.0 as f64, cam.1 as f64)
//             //             //         .trans(position.x as f64, position.y as f64),
//             //             //     graphics,
//             //             // );
//             //         }
//             //         &Renderer::Shape(ref shape) => {
//             // //             // ellipse(
//             // //             //     [1.0, 0.0, 0.0, 1.0],
//             // //             //     [0.0, 0.0, shape.rect.0 as f64, shape.rect.1 as f64],
//             // //             //     context
//             // //             //         .transform
//             // //             //         .trans(cam.0 as f64, cam.1 as f64)
//             // //             //         .trans(position.x as f64, position.y as f64),
//             // //             //     graphics,
//             // //             // );
//             //         }
//             //     }

//             // // //     // println!("Hello, {:?}", &position);
//             // }

//             let fps = fps.0;
//             let transform = context.transform.trans(10.0, 100.0);
//             text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
//                 .draw(&format!("{}fps", fps), font, &context.draw_state, transform, graphics)
//                 .unwrap();
//         });
//     }
// }
