use crate::ecs::components::Position;
use crate::ecs::components::movement::{DesiredTargetPosition, Path, TargetPosition};
use crate::ecs::resources::Adventure;
use specs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
use tatami_dungeon::Tile;
use pathfinding::prelude::astar;

pub struct PathfindingSystem;

impl<'a> System<'a> for PathfindingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, DesiredTargetPosition>,
        WriteStorage<'a, TargetPosition>,
        Read<'a, Option<Adventure>>,
    );

    fn run(&mut self, (entities, positions, desired_targets, mut targets, adventures): Self::SystemData) {
        let Some(adventure) = adventures.as_ref() else { return; };
        let floor = adventure.get_current_floor();

        // let mut to_insert = vec![];
        for (entity, pos, des_target) in (&entities, &positions, &desired_targets).join() {
            // A* needs: (start, neighbor-fn, heuristic, success-fn)
            let start = (pos.x as i32, pos.y as i32);
            let goal = (des_target.x as i32, des_target.y as i32);

            let get_neighbors = |&(x, y): &(i32, i32)| -> Vec<((i32, i32), u32)> {
                let mut neighbors = Vec::new();
                for (dx, dy) in &[
                    (0, 1), (1, 0), (0, -1), (-1, 0),
                    (1, 1), (-1, 1), (1, -1), (-1, -1),
                ] {
                    let nx = x + dx;
                    let ny = y + dy;
                    // Stay within dungeon bounds:
                    if nx < 0 || ny < 0 ||
                        nx as usize >= floor.tiles.len() ||
                        ny as usize >= floor.tiles[0].len()
                    {
                        continue;
                    }
                    if let Tile::Floor =
                        floor.tile_at(tatami_dungeon::Position {
                            x: nx as u32,
                            y: ny as u32,
                        })
                    {
                        neighbors.push(((nx, ny), 1));
                    }
                }
                neighbors
            };

            let heuristic = |&(x, y): &(i32, i32)| -> u32 {
                let dx = (x - goal.0).abs();
                let dy = (y - goal.1).abs();
                dx.max(dy) as u32
            };

            if let Some((path, _cost)) = astar(
                &start,
                get_neighbors,
                heuristic,
                |&p| p == goal
            ) {
                let steps: Vec<(u32, u32)> =
                    path.into_iter().skip(1).map(|(x, y)| (x as u32, y as u32)).collect();
                if let Some(&(next_x, next_y)) = steps.first() {
                    targets.insert(entity, TargetPosition { x: next_x, y: next_y })
                        .expect("Failed to write target position");
                }
            }
            // else: no path found, skip
        }
        
    }
}
