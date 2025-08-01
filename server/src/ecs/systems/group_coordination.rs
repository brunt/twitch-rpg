use crate::ecs::components::combat::HealthComponent;
use crate::ecs::components::{Player, Position};
use crate::ecs::resources::{Adventure, GroupDestination, RoomCheck};
use common::Health;
use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect};

pub struct GroupCoordination;

impl<'a> System<'a> for GroupCoordination {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, HealthComponent>,
        ReadExpect<'a, Option<Adventure>>,
        WriteExpect<'a, GroupDestination>,
    );

    fn run(
        &mut self,
        (players, positions, healths, adventure, mut group_destination): Self::SystemData,
    ) {
        // clean when all players are in the room that has the same id as group destination

        let Some(adventure) = adventure.as_ref() else {
            return;
        };
        let Some(dest_id) = group_destination.target_room_id else {
            return;
        };
        let Some(room) = adventure.get_room_by_id(dest_id) else {
            return;
        };

        let all_at_dest = (&players, &positions, &healths)
            .join()
            .all(|(_, position, health)| {
                health.0 != Health::Dead && room.contains(&tatami_dungeon::Position::from(position))
            });

        if all_at_dest {
            group_destination.target_room_id = None;
        }
    }
}
