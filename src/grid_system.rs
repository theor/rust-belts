use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;
use specs::prelude::*;
use specs;

use components::*;
// use piston::graphics;

pub struct System();

impl System {
    pub fn new() -> Self { System{}}
}

impl<'a> specs::prelude::System<'a> for System {
    type SystemData = (Fetch<'a, DeltaTime>,
                       FetchMut<'a, Grid>,
                       Entities<'a>,
                       WriteStorage<'a, Belt>,
                       ReadStorage<'a, Item>,
                       ReadStorage<'a, GridItem>);

    fn run(&mut self, (_delta, grid, entities, mut belt, item, grid_item): Self::SystemData) {
        use rayon::prelude::*;

        // belts -> items
        // for (_belt, belt_grid) in (&belt, &grid).join() {
        //     for (grid, _item, vel) in (&grid, &item, &mut vel).join() {
        //         if grid.ix == belt_grid.ix && grid.iy == belt_grid.iy {
        //             vel.dx = 10;
        //         }
        //     }
        // }

        (&grid_item, &item, &*entities).par_join().for_each(|(item_grid, _item, e)| {
            let mut l = (*grid).0.write().unwrap();
            (*l.entry((item_grid.ix, item_grid.iy)).or_insert(Vec::with_capacity(8))).push(e.id());
        });

        // items -> par belts
        // (&grid, &item).par_join().for_each(|(item_grid, _item)| {
        (&mut belt, &grid_item).par_join().for_each(|(belt, belt_grid)| {
            belt.items.clear();   
            for (item_grid, _item, e) in (&grid_item, &item, &*entities).join() {
                if item_grid.ix == belt_grid.ix &&
                   item_grid.iy == belt_grid.iy {
                    belt.items.push(e)
                }
            }
        });
    }
}