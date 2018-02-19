use specs;
use specs::prelude::*;

use components::*;
// use piston::graphics;

pub struct System;

impl System {
    pub fn new() -> Self {
        System {
        }
    }
}

impl<'a> specs::prelude::System<'a> for System {
    type SystemData = (Fetch<'a, DeltaTime>,
                       Fetch<'a, Grid>,
                       Entities<'a>,
                       ReadStorage<'a, Belt>,
                       ReadStorage<'a, Item>,
                       ReadStorage<'a, GridItem>,
                       WriteStorage<'a, GridVelocity>);

    fn run(&mut self, (_delta, gridq, entities, belt, item, grid, mut vel): Self::SystemData) {
        use rayon::prelude::*;

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

        
        // // (&belt, &grid).par_join().for_each(|(belt, belt_grid)| {
        // for belt in (&belt).join() {
        //     for item_id in belt.items.iter() {
        //         let mut vel = vel.get_mut(*item_id).unwrap();
        //         vel.dx = 10;
        //     }
        //  }//);

        
        // (&belt, &grid).par_join().for_each(|(belt, belt_grid)| {
        for (_belt, belt_grid) in (&belt, &grid).join() {
            let m = (*gridq).0.read().unwrap();
            if let Some(v) = m.get(&(belt_grid.ix, belt_grid.iy)) {
                for item_id in v.iter() {
                    let mut vel = vel.get_mut((*entities).entity(*item_id)).unwrap();
                    vel.dx = 10;
                    
                }
            }
        // });
        }
    }
}