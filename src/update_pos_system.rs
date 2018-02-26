use specs;
use specs::prelude::*;

use components::*;
// use piston::graphics;

pub struct System;

impl System {
    pub fn new() -> Self {
        System {}
    }
}

impl<'a> specs::prelude::System<'a> for System {
    type SystemData = (
        WriteStorage<'a, GridItem>,
        WriteStorage<'a, GridVelocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (mut grid, mut grid_vel, mut pos): Self::SystemData) {
        use rayon::prelude::*;

        (&mut grid, &mut grid_vel, &mut pos)
            .par_join()
            .for_each(|(grid, grid_vel, pos)| {
                grid.move_delta(grid_vel.dx, grid_vel.dy);
                grid_vel.dx = 0;
                grid_vel.dy = 0;

                pos.x = (grid.ix as f32 + grid.dx as f32 / 255.0) * 32f32;
                pos.y = (grid.iy as f32 + grid.dy as f32 / 255.0) * 32f32;
            });
    }
}
