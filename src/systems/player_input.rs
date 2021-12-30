use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Numpad4 => Point::new(-1, 0),
            VirtualKeyCode::Numpad6 => Point::new(1, 0),
            VirtualKeyCode::Numpad8 => Point::new(0, -1),
            VirtualKeyCode::Numpad2 => Point::new(0, 1),
            VirtualKeyCode::Numpad7 => Point::new(-1, -1),
            VirtualKeyCode::Numpad9 => Point::new(1, -1),
            VirtualKeyCode::Numpad1 => Point::new(-1, 1),
            VirtualKeyCode::Numpad3 => Point::new(1, 1),
            _ => Point::zero(),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            })
        }
    }
}
