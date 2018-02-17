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
        (&mut grid, &item).par_join().for_each(|(grid, item)| {
            let modi = self.i % 30;
            if modi == 0 {
                grid.ix += 1;
                grid.dx = 0;
            }
            else {
                grid.dx += ((1.0/30.0)*255.0) as u8;
                // println!("grid {:?} i {}", grid, self.i);
            }

        //     for (belt_pos, belt) in (&pos, &belt).join() {
        //         // position.x += delta.0 * 50f32;
        //     }
        });
    }
}