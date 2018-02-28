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
        FetchMut<'a, Grid>,
        Entities<'a>,
        ReadStorage<'a, Belt>,
        WriteStorage<'a, GridItem>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (mut tree, entities, belts, grid, pos): Self::SystemData) {
        use rayon::prelude::*;
        // use crossbeam::sync::MsQueue;
        // let queue = MsQueue::new();
        use std::sync::atomic::Ordering;

        (&*entities, &belts, &grid).par_join().for_each(|(belt_entity, belt, belt_grid)| {

            for qie in belt.items.iter() {
                    // println!("push {:?} in {:?}", qie, belt_entity);
                let item_id = qie.swap(0, Ordering::Acquire);
                if item_id == 0 {
                    continue;
                }
                let item_entity = entities.entity(item_id as u32);
                unsafe {
                    // if let None = belts.get(*item_entity) {
                    // println!("move {:?} on {:?}", item_entity, belt);
                    let ppos = pos.get(item_entity).unwrap() as *const Position;
                    let ppos = ppos as *mut Position;

                    let pgrid = grid.get(item_entity).unwrap() as *const GridItem;
                    let pgrid = pgrid as *mut GridItem;
                    let (px, py) = ((*pgrid).ix, (*pgrid).iy);

                    let (mx, my) = match &belt.direction {
                        &Direction::Right => (10,0),
                        &Direction::Down => (0,10),
                        &Direction::Up => (0, -10),
                        &Direction::Left => (-10, 0),
                    };

                    if (*pgrid).move_delta(mx, my) {
                        /*item_id = */qie.swap(0, Ordering::Release);
                        let r = GridRegion(
                            (*pgrid).ix,
                            (*pgrid).iy,
                            (*pgrid).ix + 1,
                            (*pgrid).iy + 1,
                        );
                        let mut i = 0;
                        let q = tree.0.range_query(&r);
                        for qi in q {
                            match belts.get(qi.e) {
                                None => (),
                                Some(other) => {
                                    'outer: loop {
                                        for i in 0..12 {
                                            if 0 == other.items[i].compare_and_swap(0, item_id, Ordering::Relaxed) {
                                                break 'outer;
                                            }
                                        }
                                    }
                                    break;
                                },
                            }
                            // if qi.e == belt_entity {
                            //     belt.items[i].swap(qi.e.id() as usize, Ordering::Relaxed);
                            // }
                            // i += 1;
                        }
                        // belt.items
                        // queue.push((item_entity, px, py, (*pgrid).ix, (*pgrid).iy));
                        // tx.0.remove(&RegionItem::new((*pgrid).ix, (*pgrid).iy, item_entity));
                    } else {
                        /*item_id = */qie.swap(item_id, Ordering::Release);
                    }
                    let (px, py) = (*pgrid).compute_position();
                    (*ppos).x = px;
                    (*ppos).y = py;
                    // }
                }
            }
        });
        // while let Some((item_id, px, py, nx, ny)) = queue.try_pop() {
        //     tree.0.remove(&RegionItem::new(px, py, item_id));
        //     tree.0.insert(RegionItem::new(nx, ny, item_id));
        //     // expr
        // }
        // for item_id in belt.items.iter() {
        //     unsafe {
        //         // if let None = belts.get(*item_id) {
        //         // println!("move {:?} on {:?}", item_id, belt);
        //         let ppos = pos.get(*item_id).unwrap() as *const Position;
        //         let ppos = ppos as *mut Position;

        //         let pgrid = grid.get(*item_id).unwrap() as *const GridItem;
        //         let pgrid = pgrid as *mut GridItem;
        //         (*pgrid).move_delta(10, 0);
        //         let (px, py) = (*pgrid).compute_position();
        //         (*ppos).x = px;
        //         (*ppos).y = py;
        //         // }
        //     }
        // }
        // });
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
