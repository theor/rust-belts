use specs;
use specs::{Fetch, ReadStorage, WriteStorage};

use components::*;
// use piston::graphics;

pub struct System;

impl System {
    pub fn new() -> Self {
        System {
        }
    }
}

impl<'a> specs::System<'a> for System {
    type SystemData = (Fetch<'a, DeltaTime>,
                       ReadStorage<'a, Belt>,
                       ReadStorage<'a, Item>,
                       ReadStorage<'a, GridItem>,
                       WriteStorage<'a, GridVelocity>);

    fn run(&mut self, (_delta, belt, item, grid, mut vel): Self::SystemData) {
        use rayon::prelude::*;
        use specs::ParJoin;
        use specs::Join;

        // belts -> items
        // for (_belt, belt_grid) in (&belt, &grid).join() {
        //     for (grid, _item, vel) in (&grid, &item, &mut vel).join() {
        //         if grid.ix == belt_grid.ix && grid.iy == belt_grid.iy {
        //             vel.dx = 10;
        //         }
        //     }
        // }

        // items -> par belts
        (&grid, &item, &mut vel).par_join().for_each(|(item_grid, _item, vel)| {
            for (_belt, belt_grid) in (&belt, &grid).join() {
                if item_grid.ix == belt_grid.ix &&
                   item_grid.iy == belt_grid.iy {
                    vel.dx = 10;
                }
            }
        });
    }
}