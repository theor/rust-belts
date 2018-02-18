use specs;
use specs::{Fetch, ReadStorage, WriteStorage};

use components::*;
// use piston::graphics;

pub struct System {
    i: usize,
}

impl System {
    pub fn new() -> Self {
        System {
            i: 0,
        }
    }
}

impl<'a> specs::System<'a> for System {
    type SystemData = (Fetch<'a, DeltaTime>,
                       ReadStorage<'a, Belt>,
                       ReadStorage<'a, Item>,
                       ReadStorage<'a, GridItem>,
                       WriteStorage<'a, GridVelocity>);

    fn run(&mut self, (delta, belt, item, mut grid, mut vel): Self::SystemData) {
        use rayon::prelude::*;
        use specs::ParJoin;
        use std::vec;
        self.i += 1;
        let delta_i = ((1.0/30.0)*255.0) as u16;
        
        use specs::Join;
        // let mut all_belt_items = vec::Vec::new();
        for (belt, belt_grid) in (&belt, &grid).join() {
            // let mut belt_items = vec::Vec::new();
            for (grid, item, vel) in (&grid, &item, &mut vel).join() {
                // println!("  test item {:?}", grid);
                if grid.ix == belt_grid.ix && grid.iy == belt_grid.iy {
                    // belt_items.push(item);
                    vel.dx = 10;
                    // println!("    move item {:?}", grid);
                }
            }
            // all_belt_items.push(((belt_grid.ix, belt_grid.iy), belt_items));
        }

        // for((bx,by),items) in all_belt_items {

        // }

        // for (grid, item) in (&mut grid, &item).join() {
            
        // }
                // position.x += delta.0 * 50f32;
            // }
        // (&mut grid, &item).par_join().for_each(|(grid, item)| {
        //     let ddx:u16 = grid.dx as u16 + delta_i;
        //     // println!("grid {:?} ddx {}", grid, ddx);
        //     if ddx > 255 {
        //         grid.ix += 1;
        //     }
        //     grid.dx = (ddx % 256) as u8;

        // });
    }
}