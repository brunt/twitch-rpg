// use crate::ecs::components::movement::TargetPosition;
// use crate::ecs::components::{Player, Position};
// use crate::ecs::resources::Adventure;
// use rand::seq::IteratorRandom;
// use specs::{Entities, Join, ReadExpect, ReadStorage, System, WriteStorage};

// pub struct PartySpacing;

// impl<'a> System<'a> for PartySpacing {
//     type SystemData = (
//         Entities<'a>,
//         ReadStorage<'a, Player>,
//         ReadStorage<'a, Position>,
//         WriteStorage<'a, TargetPosition>,
//         ReadExpect<'a, Option<Adventure>>,
//     );

//     fn run(&mut self, (entities, players, positions, mut targets, adventure): Self::SystemData) {
//         let Some(ref adv) = *adventure else {
//             return;
//         };

//         let party_positions: Vec<_> = (&entities, &positions, &players)
//             .join()
//             .map(|(e, p, _)| (e, p))
//             .collect();

//         for (entity, pos, _) in (&entities, &positions, &players).join() {
//             let too_close = party_positions
//                 .iter()
//                 .any(|(other_entity, other_pos)| *other_entity != entity && **other_pos == *pos);

//             if too_close {
//                 let floor = &adv.get_current_floor();
//                 let map_dimensions = (floor.tiles[0].len() as u32, floor.tiles.len() as u32);

//                 if let Some(new_pos) = tatami_dungeon::Position::from(pos)
//                     .adjacent_8(map_dimensions)
//                     .iter()
//                     .filter(|pos| floor.tile_at(**pos) != tatami_dungeon::Tile::Wall)
//                     .choose(&mut rand::rngs::ThreadRng::default())
//                 {
//                     targets.insert(entity, TargetPosition::from(new_pos)).ok();
//                 }
//             }
//         }
//     }
// }
