use specs::{Entities, Join, ReadExpect, ReadStorage, System, WriteStorage};
use crate::ecs::components::{Player, Position};
use crate::ecs::components::movement::TargetPosition;
use crate::ecs::resources::Adventure;

pub struct PartySpacing;

impl<'a> System<'a> for PartySpacing {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, TargetPosition>,
        ReadExpect<'a, Option<Adventure>>
    );

    fn run(&mut self, (entities, players, positions, mut targets, adventure): Self::SystemData) {
        let Some(ref adv) = *adventure else { return; };
        
        let party_positions: Vec<_> = (&positions, &players).join().collect();

        for (entity, pos, _) in (&entities, &positions, &players).join() {
            let too_close = party_positions.iter()
                .filter(|(other_pos, _)| *other_pos != pos)
                .any(|(other_pos, _)| pos.distance_to(other_pos) <= 1);

            if too_close {
                let floor = &adv.dungeon.floors[adv.current_floor_index];
                let map_dimensions = (floor.tiles[0].len() as u32, floor.tiles.len() as u32);
                
                if let Some(new_pos) = tatami_dungeon::Position::from(pos).adjacent_8(map_dimensions).iter().find(|pos| {
                    floor.tile_at(**pos) != tatami_dungeon::Tile::Wall
                }){
                    targets.insert(entity, TargetPosition::from(new_pos)).ok();
                }
            }
        }
    }
}