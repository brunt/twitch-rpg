use crate::ecs::components::movement::TargetPosition;
use crate::ecs::components::{Player, Position};
use crate::ecs::resources::Adventure;
use crate::ecs::resources::RoomCheck;
use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

pub struct AssignRoomTargetSystem;

impl<'a> System<'a> for AssignRoomTargetSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, TargetPosition>,
        ReadStorage<'a, Player>,
        Read<'a, Option<Adventure>>,
    );

    fn run(
        &mut self,
        (entities, positions, mut targets, players, adventure_opt): Self::SystemData,
    ) {
        let Some(adventure) = adventure_opt.as_ref() else {
            return;
        };

        let floor = adventure.get_current_floor();

        for (entity, pos, _) in (&entities, &positions, &players).join() {
            if let Some(room) = floor
                .rooms
                .iter()
                .find(|room| room.contains(&tatami_dungeon::Position::from(pos)))
            {
                let target = if let Some(stair) = room.stairs.first() {
                    TargetPosition::from(&stair.position)
                } else {
                    TargetPosition {
                        x: room.position.x + room.width / 2,
                        y: room.position.y + room.height / 2,
                    }
                };

                targets.insert(entity, target).ok();
            }
        }
    }
}
