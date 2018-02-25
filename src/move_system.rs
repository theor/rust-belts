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
        ReadStorage<'a, Belt>,
        WriteStorage<'a, GridItem>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (belts, mut grid, mut pos): Self::SystemData) {
        use rayon::prelude::*;
        (&belts).par_join().for_each(|belt| {
            for item_id in belt.items.iter() {
                unsafe {
                    // if let None = belts.get(*item_id) {
                    // println!("move {:?} on {:?}", item_id, belt);
                    let ppos = pos.get(*item_id).unwrap() as *const Position;
                    let ppos = ppos as *mut Position;

                    let pgrid = grid.get(*item_id).unwrap() as *const GridItem;
                    let pgrid = pgrid as *mut GridItem;
                    (*pgrid).move_delta(10, 0);
                    let (px, py) = (*pgrid).compute_position();
                    (*ppos).x = px;
                    (*ppos).y = py;
                    // }
                }

                // match vel.get(*item_id) {
                //     Some (vel) => {
                //         let pvel = vel as *const GridVelocity;
                //         unsafe {
                //             let mpvel = pvel as *mut GridVelocity;
                //             (*mpvel).dx = 10;
                //         }
                //     },
                //     None => (),
                // }
            }
        });
    }
}
// impl<'a> specs::prelude::System<'a> for System {
//     type SystemData = (ReadStorage<'a, Belt>,
//                        WriteStorage<'a, GridVelocity>);

//     fn run(&mut self, (belt, mut vel): Self::SystemData) {
//         use rayon::prelude::*;

//         // belts -> items
//         // for (_belt, belt_grid) in (&belt, &grid).join() {
//         //     for (grid, _item, vel) in (&grid, &item, &mut vel).join() {
//         //         if grid.ix == belt_grid.ix && grid.iy == belt_grid.iy {
//         //             vel.dx = 10;
//         //         }
//         //     }
//         // }

//         // items -> par belts
//         // (&grid, &item, &mut vel).par_join().for_each(|(item_grid, _item, vel)| {
//         //     for (_belt, belt_grid) in (&belt, &grid).join() {
//         //         if item_grid.ix == belt_grid.ix &&
//         //            item_grid.iy == belt_grid.iy {
//         //             vel.dx = 10;
//         //         }
//         //     }
//         // });

//         // // (&belt, &grid).par_join().for_each(|(belt, belt_grid)| {
//         // for belt in (&belt).join() {
//         //     for item_id in belt.items.iter() {
//         //         let mut vel = vel.get_mut(*item_id).unwrap();
//         //         vel.dx = 10;
//         //     }
//         //  }//);

//         // (&belt, &grid).par_join().for_each(|(belt, belt_grid)| {
//         (&belt).par_join().for_each(|belt| {
//             for item_id in belt.items.iter() {
//                 match vel.get(*item_id) {
//                     Some (vel) => {
//                         let pvel = vel as *const GridVelocity;
//                         unsafe {
//                             let mpvel = pvel as *mut GridVelocity;
//                             (*mpvel).dx = 10;
//                         }
//                     },
//                     None => (),
//                 }
//             }
//          });

//         // let mut vr = vel.par_restrict_mut();

//         // (&belt, &grid).par_join().for_each(|(belt, belt_grid)| {
//         // for (_belt, belt_grid) in (&belt, &grid).join() {
//         //     let m = (*gridq).0.read().unwrap();
//         //     if let Some(v) = m.get(&(belt_grid.ix, belt_grid.iy)) {
//         //         let mut items = BitSet::new();
//         //         for item_id in v.iter() {
//         //             items.add(*item_id);
//         //         }
//         //         for (e,(v,mut s)) in (&items, &mut vr).join() {
//         //             let vel = s.get_mut_unchecked(&v);
//         //             vel.dx = 10;

//         //         }
//         //     }
//         // });
//         // }
//     }
// }
