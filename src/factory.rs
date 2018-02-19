use specs::prelude::*;
use components::*;

pub fn init(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderer>();
    world.register::<Belt>();
    world.register::<Item>();
    world.register::<GridItem>();
    world.register::<GridVelocity>();

    world.add_resource(DeltaTime(0f32));
    world.add_resource(Camera(0f32, 0f32));
}

pub fn belt(world: &mut World, x: u32, y: u32) -> Entity {
    world.create_entity()
            .with(Position::new())
            .with(GridItem::new(x, y))
            .with(Renderer::sprite("transport-belt.png", (0u8,0u8)))
            .with(Belt::new())
            .build()
}

pub fn item(world: &mut World, x: u32, y: u32) -> Entity {
    world.create_entity()
        .with(Position::new())
        // .with(Renderer::shape((16u8,16u8)))
        .with(Renderer::sprite("copper-plate.png", (0u8,0u8)))
        .with(GridItem::new(x, y))
        .with(GridVelocity::new())
        .with(Item{})
        .build()
}