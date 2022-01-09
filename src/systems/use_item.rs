use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[read_component(ProvidesDungeonMap)]
#[write_component(Health)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            let item = ecs.entry_ref(activate.item);
            let mut used_effect = false;
            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((activate.used_by, healing.amount));
                    used_effect = true;
                }
                if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                    used_effect = true;
                }
            }
            if used_effect {
                commands.remove(activate.item);
            }
            commands.remove(*entity);
        });

    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal.1);
            }
        }
    }
}
