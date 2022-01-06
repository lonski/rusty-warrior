use super::MapArchitect;
use crate::prelude::*;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };
        self.random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            self.iteration(&mut mb.map);
        }
        let start = self.find_start(&mb.map);
        mb.monster_spawns = mb.spawn_monsters(&start, rng);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();

        mb
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let size = map.tiles.len();
        for (idx, tile) in map.tiles.iter_mut().enumerate() {
            if idx <= SCREEN_WIDTH as usize
                || idx > size - SCREEN_WIDTH as usize
                || idx % SCREEN_WIDTH as usize == 0
                || idx % SCREEN_WIDTH as usize == SCREEN_WIDTH as usize - 1
            {
                *tile = TileType::Wall;
            } else if rng.range(0, 100) > 55 {
                *tile = TileType::Floor;
            } else {
                *tile = TileType::Wall;
            }
        }
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut nb = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if ix == 0 && iy == 0 {
                    continue;
                }
                if map.tiles[map_idx(x + ix, y + iy)] == TileType::Wall {
                    nb += 1;
                }
            }
        }
        nb
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 2..SCREEN_HEIGHT - 2 {
            for x in 2..SCREEN_WIDTH - 2 {
                let neighbors = self.count_neighbors(x, y, map);
                let idx = map_idx(x, y);
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
                )
            })
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(distance2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        map.index_to_point2d(closest_point)
    }
}
