use specs::LazyUpdate;
use specs;
use specs::{Fetch, ReadStorage, WriteStorage, Entities};

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
                       WriteStorage<'a, GridVelocity>,
                       Fetch<'a, LazyUpdate>);

    fn run(&mut self, (_delta, belt, item, grid, mut vel, updater): Self::SystemData) {
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
        // (&grid, &item, &mut vel).par_join().for_each(|(item_grid, _item, vel)| {
        //     for (_belt, belt_grid) in (&belt, &grid).join() {
        //         if item_grid.ix == belt_grid.ix &&
        //            item_grid.iy == belt_grid.iy {
        //             vel.dx = 10;
        //         }
        //     }
        // });

        
        (&belt, &grid).par_join().for_each(|(belt, belt_grid)| {
                for item_id in belt.items.iter() {
                    updater.insert(*item_id, GridVelocity { dx: 10, dy: 0});
                }
        });
        // for belt in (&belt).join() {
        //     for item_id in belt.items.iter() {
        //         let mut vel = vel.get_mut(*item_id).unwrap();
        //         vel.dx = 10;
        //     }
        //  }//);reads
    }
}