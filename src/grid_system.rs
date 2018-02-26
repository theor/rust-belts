use specs::prelude::*;
use specs;

use components::*;
pub struct System;

impl System {
    pub fn new() -> Self { System{} }
}

impl<'a> specs::prelude::System<'a> for System {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        FetchMut<'a, Grid>,
        Entities<'a>,
        ReadStorage<'a, GridItem>,
    );

    fn run(&mut self, (_delta, mut tree, entities, grid_item): Self::SystemData) {
        // use rayon::prelude::*;

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
                tree.0
                    .insert(RegionItem::new(belt_grid.ix, belt_grid.iy, belt_entity));
            });
    }
}
