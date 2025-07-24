use crate::ecs::components::Position;
use crate::ecs::components::movement::TargetPosition;
use crate::ecs::resources::Adventure;
use specs::{Entities, Join, ReadExpect, ReadStorage, System, WriteStorage};
use tatami_dungeon::Tile;

pub struct MovementValidationSystem;

impl<'a> System<'a> for MovementValidationSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, TargetPosition>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, Option<Adventure>>,
    );

    fn run(&mut self, (entities, mut targets, positions, adventure_maybe): Self::SystemData) {
        let Some(adventure) = adventure_maybe.as_ref() else {
            return;
        };
        let map = adventure.get_current_floor();

        let mut to_remove = Vec::new();

        for (entity, target_pos, position) in (&entities, &mut targets, &positions).join() {
            if !is_clear_path(
                map,
                tatami_dungeon::Position {
                    x: position.x,
                    y: position.y,
                },
                tatami_dungeon::Position {
                    x: target_pos.x,
                    y: target_pos.y,
                },
            ) {
                //     Cancel the movement intent
                to_remove.push(entity)
            }
        }

        for entity in to_remove {
            targets.remove(entity);
        }
    }
}

pub fn is_clear_path(
    floor: &tatami_dungeon::Floor,
    from: tatami_dungeon::Position,
    to: tatami_dungeon::Position,
) -> bool {
    for (x, y) in walk_line(from, to) {
        if x < 0 || y < 0 || y as usize >= floor.tiles.len() || x as usize >= floor.tiles[0].len() {
            return false;
        }

        if floor.tiles[y as usize][x as usize] != Tile::Floor {
            return false;
        }
    }

    true
}

pub fn walk_line(from: tatami_dungeon::Position, to: tatami_dungeon::Position) -> Vec<(i32, i32)> {
    let (mut x0, mut y0) = (from.x as i32, from.y as i32);
    let (x1, y1) = (to.x as i32, to.y as i32);

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;

    let mut path = vec![(x0, y0)];

    while x0 != x1 || y0 != y1 {
        let e2 = 2 * err;

        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }

        if e2 < dx {
            err += dx;
            y0 += sy;
        }

        path.push((x0, y0));
    }

    path
}
