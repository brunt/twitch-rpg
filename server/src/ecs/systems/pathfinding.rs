use crate::ecs::components::Position;
use crate::ecs::components::movement::{Path, TargetPosition};
use crate::ecs::resources::Adventure;
use specs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
use tatami_dungeon::Tile;

pub struct PathfindingSystem;

impl<'a> System<'a> for PathfindingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, TargetPosition>,
        WriteStorage<'a, Path>,
        Read<'a, Option<Adventure>>,
    );

    fn run(&mut self, (entities, positions, targets, mut paths, adventures): Self::SystemData) {
        let Some(adventure) = adventures.as_ref() else {
            return;
        };

        let mut new_paths = Vec::new();

        for (entity, pos, target, _) in (&entities, &positions, &targets, !&paths).join() {
            let mut x = pos.x;
            let mut y = pos.y;
            let mut path = Vec::new();
            let mut blocked = false;

            while x != target.x || y != target.y {
                // Move 1 tile in direction of target (Chebyshev-style)
                if x < target.x {
                    x += 1;
                } else if x > target.x {
                    x -= 1;
                }

                if y < target.y {
                    y += 1;
                } else if y > target.y {
                    y -= 1;
                }

                if matches!(
                    adventure.dungeon.floors[adventure.current_floor_index]
                        .tile_at(tatami_dungeon::Position { x, y }),
                    Tile::Floor
                ) {
                    path.push((x, y));
                } else {
                    blocked = true;
                    break;
                }

                path.push((x, y));
            }
            if !blocked {
                new_paths.push((entity, Path { steps: path }));
            }

            // Attach path to the entity
        }

        for (entity, path) in new_paths {
            paths.insert(entity, path).expect("Failed to insert path");
        }
    }
}
