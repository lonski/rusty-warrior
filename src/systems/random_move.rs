use crate::prelude::*;

#[system]
#[read_component(MovingRandomly)]
#[read_component(Point)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    let mut rng = RandomNumberGenerator::new();
    movers.iter_mut(ecs).for_each(|(entity, pos, _)| {
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;
        commands.push((
            (),
            WantsToMove {
                entity: *entity,
                destination,
            },
        ));
    });
}