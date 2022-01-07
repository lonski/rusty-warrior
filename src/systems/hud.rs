use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Name)]
#[read_component(Carried)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).next().unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );

    //inventory
    let player = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .find_map(|entity| Some(*entity))
        .unwrap();
    let mut y = 3;
    let mut item_query = <(&Item, &Name, &Carried)>::query();
    item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(Point::new(3, y), format!("{}: {}", y - 2, &name.0));
            y += 1;
        });
    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(10000).expect("Batch error");
}
