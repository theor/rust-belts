use specs;
use specs::{ReadStorage, WriteStorage, Entities, LazyUpdate, Fetch};

use components::*;
// use piston::graphics;

pub struct System;

impl<'a> specs::System<'a> for System {
    type SystemData = (Entities<'a>,
                       WriteStorage<'a, GridItem>,
                       ReadStorage<'a, GridVelocity>,
                       WriteStorage<'a, Position>,
                       Fetch<'a, LazyUpdate>);

    fn run(&mut self, (entities, mut grid, grid_vel, mut pos, updater): Self::SystemData) {
        use rayon::prelude::*;
        use specs::ParJoin;

        (&*entities, &mut grid, &grid_vel)
          .par_join()
          .for_each(|(e, grid, grid_vel)|{
              grid.move_delta(grid_vel.dx, grid_vel.dy);
              updater.remove::<GridVelocity>(e);
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