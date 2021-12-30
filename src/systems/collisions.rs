use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let player_pos: Point = *<&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap_or(&Point::zero());

    <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(entity, _)| commands.remove(*entity));
}
