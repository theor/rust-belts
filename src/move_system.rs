use specs;
use specs::{Fetch, WriteStorage};

use components::{DeltaTime, Position};
// use piston::graphics;

pub struct System;

impl<'a> specs::System<'a> for System {
    type SystemData = (Fetch<'a, DeltaTime>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, (delta, mut data): Self::SystemData) {
        use specs::Join;
        for position in (&mut data).join() {
            position.x += delta.0 * 50f32;
        }
    }
}