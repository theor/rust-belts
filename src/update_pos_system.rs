use specs;
use specs::{Fetch, ReadStorage, WriteStorage, Entities};

use components::*;
// use piston::graphics;

pub struct System;

impl<'a> specs::System<'a> for System {
    type SystemData = (WriteStorage<'a, GridItem>,
                       WriteStorage<'a, GridVelocity>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, (mut grid, mut grid_vel, mut pos): Self::SystemData) {
        use rayon::prelude::*;
        use specs::ParJoin;

        (&mut grid, &mut grid_vel)
          .par_join()
          .for_each(|(grid, grid_vel)|{
              grid.move_delta(grid_vel.dx, grid_vel.dy);
              grid_vel.dx = 0;
              grid_vel.dy = 0;
          });
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