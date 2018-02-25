#![feature(test)]

extern crate find_folder;
extern crate flame;
extern crate fps_counter;
extern crate graphics;
extern crate piston_window;
extern crate rayon;
extern crate specs;
extern crate ntree;
// extern crate sprite;
#[macro_use]
extern crate specs_derive;

#[cfg(test)]
extern crate test;

mod components;
mod render_system;
mod move_system;
mod update_pos_system;
mod grid_system;
mod resmgr;
mod factory;
// mod quadtree;


use fps_counter::FPSCounter;
use piston_window::*;
use specs::prelude::{DispatcherBuilder, World, RunNow};
use components::{FPS, DeltaTime, Grid};

fn main() {
    let mut world = World::new();
    factory::init(&mut world);
    
    factory::belt(&mut world, 0, 0);
    factory::item_subpos(&mut world, 1, 1, 0, 0);
    // 10k belts, 80k items: 60fps
    //  for j in 0..1000 {
    //         for i in 0..100 {
    //             factory::belt(&mut world, i, j);
    //         }
    //         for i in 0..250 {
    //             for d in 0..4 {
    //                 factory::item_subpos(&mut world, i, j, d * (255 / 4), 0);
    //                 factory::item_subpos(&mut world, i, j, d * (255 / 4),127);
    //             }
    //         }
    //     }
    println!("Init done");
    // for i in 0..10 {
    //     factory::belt(&mut world, i, 0);
    // }
    // factory::item(&mut world, 0, 0);
    // factory::item(&mut world, 1, 0);
    // factory::item(&mut world, 0, 1);

    let mut dispatcher = DispatcherBuilder::new();
        dispatcher.add(move_system::System::new(), "move", &[]);
        // dispatcher.add(update_pos_system::System, "update_pos_system", &["move"]);
    let mut dispatcher = dispatcher.build();

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut mgr = resmgr::ResMgr::new();
    {
        mgr.load(&mut window.factory, "transport-belt.png", 16, (40,40), (4,4));
        mgr.load(&mut window.factory, "copper-plate.png", 1, (32,32), (0,0));
    }
    let ref font = mgr.asset_path("FiraSans-Regular.ttf");
    world.add_resource(mgr);
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
    world.add_resource(FPS(0));
    world.add_resource(Grid::new());
    // let image   = Image::new().rect(graphics::rectangle::square(0.0, 0.0, 200.0));
    
    println!("Grid System");
    grid_system::System::new().run_now(&mut world.res);
    let mut counter = FPSCounter::new();
    println!("Start loop");

    while let Some(event) = window.next() {

        if let Some(_r) = event.render_args() {

             {
                let mut delta = world.write_resource::<FPS>();
                *delta = FPS(counter.tick());
            }
            // let _guard = flame::start_guard("render");
            let mut render = render_system::System(&mut window, &event);
            let data = render.fetch(&mut world);
            render.run(data, &mut glyphs);
            // render.run_now(&mut world.res);

        }

        if let Some(u) = event.update_args() {
            {
                let mut delta = world.write_resource::<DeltaTime>();
                *delta = DeltaTime(u.dt as f32);
            }
            let _guard = flame::start_guard("update");
            dispatcher.dispatch(&mut world.res);
        }
    }

    use std::fs::File;
    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::components::*;
    use test::Bencher;

    #[test]
    pub fn ntree_tests() {
        let mut world = World::new();

        use ntree::{Region, NTree};
        let r = GridRegion(0,0,128,128);
        let gizero = RegionItem::new(0, 0, world.create_entity().build());
        assert_eq!(true, r.contains(&gizero));
        assert_eq!(false, r.contains(&RegionItem::new(128, 128, world.create_entity().build())));
        let mut tree = NTree::<GridRegion, RegionItem>::new(r, 8);
        assert_eq!(true, tree.contains(&gizero));
        assert_eq!(true, tree.contains(&RegionItem::new(32, 0, world.create_entity().build())));
        tree.insert(RegionItem::new(0, 0, world.create_entity().build()));
        tree.insert(RegionItem::new(32, 0, world.create_entity().build()));
        assert_eq!(true, tree.contains(&gizero));
        assert_eq!(true, tree.contains(&RegionItem::new(32, 0, world.create_entity().build())));

        let q = tree.range_query(&GridRegion(0,0,2,2)).collect::<Vec<&RegionItem>>();
        assert_eq!(1, q.len());
        
        let q = tree.range_query(&GridRegion(1,1,2,2)).collect::<Vec<&RegionItem>>();
        assert_eq!(0, q.len());
        
        let q = tree.range_query(&GridRegion(0,0,33,33)).collect::<Vec<&RegionItem>>();
        assert_eq!(2, q.len());
    }

    #[bench]
    pub fn bench_vec(b: &mut Bencher) {
        let mut vels: Vec<GridVelocity> = (0..1000*1000).map(|i| { GridVelocity::new() }).collect();
        b.iter(|| {
            vels.iter_mut().for_each(|vel| {
            // for vel in  {
                vel.dx += 1;
            });
        });
    } 
    #[bench]
    pub fn bench_vec_mutptr(b: &mut Bencher) {
        let mut vels: Vec<GridVelocity> = (0..1000*1000).map(|i| { GridVelocity::new() }).collect();
        b.iter(|| {
            let ptr = vels.as_mut_ptr();
            for i in (0..1000*1000) {
                unsafe {
                    let vel = ptr.offset((i as isize));
                    (*vel).dx += 1;
                }
            };
        });
    } 
    #[bench]
    pub fn bench_vec_par_mutptr(b: &mut Bencher) {
        let mut vels: Vec<GridVelocity> = (0..1000*1000).map(|i| { GridVelocity::new() }).collect();
        b.iter(|| {
            let ptr = vels.as_mut_ptr();
            for i in (0..1000*1000) {
                unsafe {
                    let vel = ptr.offset((i as isize));
                    (*vel).dx += 1;
                }
            };
        });
    } 

    #[bench]
    pub fn bench_vec_for(b: &mut Bencher) {
        let mut vels: Vec<GridVelocity> = (0..1000*1000).map(|i| { GridVelocity::new() }).collect();
        b.iter(|| {
            for vel in vels.iter_mut() {
            // for vel in  {
                vel.dx += 1;
            };
        });
    } 

    #[bench]
    pub fn bench_vec_par(b: &mut Bencher) {
        let mut vels: Vec<GridVelocity> = (0..1000*1000).map(|i| { GridVelocity::new() }).collect();
        use rayon::prelude::*;
        b.iter(|| {
            vels.par_iter_mut().for_each(|vel| {
            // for vel in  {
                vel.dx += 1;
            });
        });
    } 
    
    #[bench]
    pub fn bench_storage(b: &mut Bencher) {
        let mut world = setup_world();
        let mut vel_storage = world.write::<GridVelocity>();
        let ents = world.entities();
        b.iter(|| {
        for i in 0..1000*1000 {
            let e = ents.entity(i);
        // for vel in  {
            let mut vel = vel_storage.get_mut(e).unwrap();
            vel.dx += 1;
        }
        });
    }    
    
    #[bench]
    pub fn bench_vecstorage(b: &mut Bencher) {
        use std::default::Default;
        use specs::prelude::*;
        use specs::prelude::VecStorage;
        use specs::storage::UnprotectedStorage;

        let mut vel_storage: VecStorage<GridVelocity> = Default::default();
        for i in 0..1000*1000 {
            unsafe { vel_storage.insert(i, GridVelocity::new()); }
        }
        b.iter(|| {
        for i in 0..1000*1000 {
        // for vel in  {
            let mut vel = unsafe { vel_storage.get_mut(i) };
            vel.dx += 1;
        }
        });
    }  
    
    #[bench]
    pub fn bench_storage_entity_prefetch(b: &mut Bencher) {
        use specs::prelude::Entity;
        let mut world = setup_world();
        let mut vel_storage = world.write::<GridVelocity>();
        let ents = world.entities();
        let entities: Vec<Entity> = (0..1000*1000).map(|i| ents.entity(i)).collect();
        b.iter(|| {
        for e in entities.iter() {
        // for vel in  {
            let mut vel = vel_storage.get_mut(*e).unwrap();
            vel.dx += 1;
        }
        });
    }   

    #[bench]
    pub fn bench_storage_par(b: &mut Bencher) {
        let mut world = setup_world();
        let mut vel_storage = world.write::<GridVelocity>();
        let ents = world.entities();
        use rayon::prelude::*;
        let ids:Vec<u32> = (0..1000*1000).collect();
        b.iter(|| {
            ids.par_iter().for_each(|i| {
                let e = ents.entity(*i);
            // for vel in  {
                unsafe {
                    let vel = vel_storage.get(e).unwrap();
                    let vel = vel as *const GridVelocity;
                    let vel = vel as *mut GridVelocity;
                    (*vel).dx += 1;
                }
            });
        });
    }

    fn setup_world() -> World {
        let mut world = World::new();
        factory::init(&mut world);
        world.add_resource(Grid::new());
        
       for j in 0..1000 {
           for i in 0..250 {
                for d in 0..4 {
                    factory::item_subpos(&mut world, i, j, d * (255 / 4), 0);
                    factory::item_subpos(&mut world, i, j, d * (255 / 4),127);
                }
            }
       }
       for j in 0..100 {
            for i in 0..100 {
                factory::belt(&mut world, i, j);
            }
        }
        world
    }

    #[bench]
    pub fn bench(b: &mut Bencher) {
        let mut world = setup_world();
        
        grid_system::System::new().run_now(&mut world.res);

        let mut dispatcher = DispatcherBuilder::new();
        dispatcher.add(move_system::System::new(), "move", &[]);
        dispatcher.add(update_pos_system::System, "update_pos_system", &["move"]);
        let mut dispatcher = dispatcher.build();

        b.iter(||{
            move_system::System::new().run_now(&mut world.res);
            // dispatcher.dispatch(&mut world.res);
        });
    }

        #[bench]
    pub fn bench_updatepos(b: &mut Bencher) {
        let mut world = setup_world();
        
        grid_system::System::new().run_now(&mut world.res);

        let mut dispatcher = DispatcherBuilder::new();
        dispatcher.add(move_system::System::new(), "move", &[]);
        dispatcher.add(update_pos_system::System, "update_pos_system", &["move"]);
        let mut dispatcher = dispatcher.build();

        b.iter(||{
            update_pos_system::System::new().run_now(&mut world.res);
            // dispatcher.dispatch(&mut world.res);
        });
    }
}