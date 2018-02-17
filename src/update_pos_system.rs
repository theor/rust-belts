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
              pos.x = grid.ix as f32 * 32f32;
              pos.y = grid.iy as f32 * 32f32;
            // position.x += delta.0 * 50f32;
          });
    }
}