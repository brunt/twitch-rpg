// use std::collections::{HashMap, VecDeque};
// use specs::{System, ReadStorage, WriteStorage, ReadExpect, Join};
// use tatami_dungeon::Floor;
// use crate::ecs::components::movement::{Path, TargetPosition};
// use crate::ecs::components::Position;
// use crate::ecs::resources::Adventure;
//
// pub struct PlayerPathfindingSystem;
//
// impl<'a> System<'a> for PlayerPathfindingSystem {
//     type SystemData = (
//         ReadStorage<'a, Position>,
//         ReadStorage<'a, TargetPosition>,
//         WriteStorage<'a, Path>,
//         ReadExpect<'a, Option<Adventure>>,
//     );
//
//     fn run(&mut self, (players, positions, targets, mut paths, adventure): Self::SystemData) {
//         let floor = &adventure.dungeon.floors[adventure.current_floor_index].tiles;
//
//         for (_, pos, target, path) in (&players, &positions, &targets, &mut paths).join() {
//             if path.steps.is_empty() {
//                 let start = (pos.x as i32, pos.y as i32);
//                 let end = (target.x as i32, target.y as i32);
//
//                 if let Some(new_path) = bfs_path(floor, start, end) {
//                     path.steps = new_path;
//                 }
//             }
//         }
//     }
// }
//
// fn bfs_path(
//     floor: &Floor,
//     start: (i32, i32),
//     goal: (i32, i32),
// ) -> Option<Vec<(u32, u32)>> {
//     let mut frontier = VecDeque::new();
//     frontier.push_back(start);
//
//     let mut came_from = HashMap::new();
//     came_from.insert(start, None);
//
//     let directions = [
//         (-1, 0), (1, 0), (0, -1), (0, 1), // cardinal
//         (-1, -1), (-1, 1), (1, -1), (1, 1), // diagonals
//     ];
//
//     while let Some(current) = frontier.pop_front() {
//         if current == goal {
//             break;
//         }
//
//         for (dx, dy) in directions {
//             let next = (current.0 + dx, current.1 + dy);
//             if is_walkable(floor, next.0, next.1) && !came_from.contains_key(&next) {
//                 frontier.push_back(next);
//                 came_from.insert(next, Some(current));
//             }
//         }
//     }
//
//     if !came_from.contains_key(&goal) {
//         return None;
//     }
//
//     // Reconstruct path
//     let mut path = vec![];
//     let mut current = goal;
//     while let Some(prev) = came_from[&current] {
//         path.push((current.0 as u32, current.1 as u32));
//         current = prev;
//     }
//
//     path.reverse();
//     Some(path)
// }
