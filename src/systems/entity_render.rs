use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut player = <(&FieldOfView, &Point, &Render)>::query().filter(component::<Player>());
    let player_fov = player.iter(ecs).next().unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);
    <(&Point, &Render)>::query()
        .iter(ecs)
        .filter(|(pos, _)| player_fov.0.visible_tiles.contains(pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    // Draw player at the end to stand over items
    draw_batch.set(
        *player_fov.1 - offset,
        player_fov.2.color,
        player_fov.2.glyph,
    );

    draw_batch.submit(5000).expect("Batch error");
}
