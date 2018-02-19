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
                       Entities<'a>,
                       WriteStorage<'a, Belt>,
                       ReadStorage<'a, Item>,
                       ReadStorage<'a, GridItem>);

    fn run(&mut self, (_delta, entities, mut belt, item, grid): Self::SystemData) {
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
        (&mut belt, &grid).par_join().for_each(|(belt, belt_grid)| {
            belt.items.clear();   
            for (item_grid, _item, e) in (&grid, &item, &*entities).join() {
                if item_grid.ix == belt_grid.ix &&
                   item_grid.iy == belt_grid.iy {
                    belt.items.push(e)
                }
            }
        });
    }
}