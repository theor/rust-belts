use specs;
use specs::{VecStorage, WriteStorage};

use components::Position;
// use piston::graphics;

pub struct System;

impl<'a> specs::System<'a> for System {
    type SystemData = WriteStorage<'a, Position>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;
        for position in (&mut data).join() {
            position.x += 1f32;
        }
    }
}