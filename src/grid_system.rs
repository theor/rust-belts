use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;
use specs::prelude::*;
use specs;

use components::*;
// use piston::graphics;
use ntree::{NTree, Region};
type GridTree = NTree<GridRegion, RegionItem>;
pub struct System(GridTree);

impl System {
    pub fn new() -> Self {
        let r = GridRegion(0, 0, 1024, 1024);
        System(GridTree::new(r, 10))
    }
}

impl<'a> specs::prelude::System<'a> for System {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        Entities<'a>,
        WriteStorage<'a, Belt>,
        ReadStorage<'a, Item>,
        ReadStorage<'a, GridItem>,
    );

    fn run(&mut self, (_delta, entities, mut belt, item, grid_item): Self::SystemData) {
        use rayon::prelude::*;

        // belts -> items
        // for (_belt, belt_grid) in (&belt, &grid).join() {
        //     for (grid, _item, vel) in (&grid, &item, &mut vel).join() {
        //         if grid.ix == belt_grid.ix && grid.iy == belt_grid.iy {
        //             vel.dx = 10;
        //         }
        //     }
        // }

        // (&grid_item, &item, &*entities).par_join().for_each(|(item_grid, _item, e)| {
        //     let mut l = (*grid).0.write().unwrap();
        //     (*l.entry((item_grid.ix, item_grid.iy)).or_insert(Vec::with_capacity(8))).push(e.id());
        // });

        (&*entities, &grid_item)
            .join()
            .for_each(|(belt_entity, belt_grid)| {
                // println!("insert {:?} at {:?}", belt_entity, belt_grid);
                self.0
                    .insert(RegionItem::new(belt_grid.ix, belt_grid.iy, belt_entity));
            });
        (&*entities, &mut belt, &grid_item).par_join().for_each(
            |(belt_entity, belt, belt_grid)| {
                let r = GridRegion(
                    belt_grid.ix,
                    belt_grid.iy,
                    belt_grid.ix + 1,
                    belt_grid.iy + 1,
                );
                let q = self.0.range_query(&r);
                belt.items.clear();
                for qi in q {
                    if qi.e != belt_entity {
                        // println!("push {:?} in {:?}", qi.e, belt_entity);
                        belt.items.push(qi.e);
                    }
                }
            },
        );
    }
}
