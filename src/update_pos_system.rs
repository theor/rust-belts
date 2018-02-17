use specs;
use specs::{Fetch, ReadStorage, WriteStorage};

use components::*;
// use piston::graphics;

pub struct System;

impl<'a> specs::System<'a> for System {
    type SystemData = (ReadStorage<'a, GridItem>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, (grid, mut pos): Self::SystemData) {
        use rayon::prelude::*;
        use specs::ParJoin;
        (&grid, &mut pos)
          .par_join()
          .for_each(|(grid, pos)|{
              pos.x = (grid.ix as f32 + grid.dx as f32 / 255.0) * 32f32;
              pos.y = (grid.iy as f32 + grid.dy as f32 / 255.0) * 32f32;
            //   println!("grid {:?} pos {:?}", grid, pos);
            // position.x += delta.0 * 50f32;
          });
    }
}