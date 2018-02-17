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
                       WriteStorage<'a, GridItem>);

    fn run(&mut self, (delta, belt, item, mut grid): Self::SystemData) {
        use rayon::prelude::*;
        use specs::ParJoin;
        self.i += 1;
        let delta_i = ((1.0/30.0)*255.0) as u16;
        (&mut grid, &item).par_join().for_each(|(grid, item)| {
            let ddx:u16 = grid.dx as u16 + delta_i;
            // println!("grid {:?} ddx {}", grid, ddx);
            if ddx > 255 {
                grid.ix += 1;
            }
            grid.dx = (ddx % 256) as u8;

        //     for (belt_pos, belt) in (&pos, &belt).join() {
        //         // position.x += delta.0 * 50f32;
        //     }
        });
    }
}