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
    world.add_resource(Grid(GridTree::new(GridRegion(0,0,1024,1024), 12)));
}

pub fn belt(world: &mut World, x: u32, y: u32, d: Direction) -> Entity {
    let grid = GridItem::new(x, y);
    let (px, py) = grid.compute_position();
    use components::Direction::*;
    use components::Flip::*;
    let (src, s) = match d.clone() {
        Right => ((0u8, 0u8), (1.0, 1.0)),
        Down => ((0u8, 40u8), (1.0, -1.0)),
        Up => ((0u8, 40u8), (1.0, 1.0)),
        Left => ((0u8, 0u8), (-1.0, 1.0)),
    };
    world
        .create_entity()
        .with(Position::at(px, py))
        .with(grid)
        .with(Renderer::sprite(0, src, s, None))
        .with(Belt::new(d.clone()))
        .build()
}

pub fn item_subpos(world: &mut World, x: u32, y: u32, dx: u8, dy: u8) -> Entity {
    use components;
    let grid = GridItem::new_subpos(x, y, dx, dy);
    let (px, py) = grid.compute_position();
    world.create_entity()
        .with(Position::at(px, py))
        .with(grid)
        // .with(Renderer::shape((16u8,16u8)))
        .with(Renderer::sprite(1, (0u8,0u8), (0.5,0.5), components::Flip::None))
        .with(GridVelocity::new())
        .with(Item{})
        .build()
}
#[allow(dead_code)]
pub fn item(world: &mut World, x: u32, y: u32) -> Entity {
    use components;
    let grid = GridItem::new(x, y);
    let (px, py) = grid.compute_position();
    world.create_entity()
        .with(Position::at(px, py))
        .with(grid)
        // .with(Renderer::shape((16u8,16u8)))
        .with(Renderer::sprite(1, (0u8,0u8), (0.5,0.5), components::Flip::None))
        .with(GridVelocity::new())
        .with(Item{})
        .build()
}
