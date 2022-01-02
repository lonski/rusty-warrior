use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let current_state = *turn_state;
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    let mut player = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_pos = amulet.iter(ecs).next().unwrap();

    player.iter(ecs).for_each(|(hp, pos)| {
        if pos == amulet_pos {
            new_state = TurnState::Victory;
        }
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
    });

    *turn_state = new_state;
}
