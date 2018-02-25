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
            .with(Renderer::sprite(0, (40u8,40u8)))
            .with(Belt::new())
            .build()
}

pub fn item_subpos(world: &mut World, x: u32, y: u32, dx: u8, dy: u8) -> Entity {
    world.create_entity()
        .with(Position::new())
        // .with(Renderer::shape((16u8,16u8)))
        .with(Renderer::sprite(1, (20u8,20u8)))
        .with(GridItem::new_subpos(x, y, dx, dy))
        .with(GridVelocity::new())
        .with(Item{})
        .build()
}
pub fn item(world: &mut World, x: u32, y: u32) -> Entity {
    world.create_entity()
        .with(Position::new())
        // .with(Renderer::shape((16u8,16u8)))
        .with(Renderer::sprite(1, (20u8,20u8)))
        .with(GridItem::new(x, y))
        .with(GridVelocity::new())
        .with(Item{})
        .build()
}