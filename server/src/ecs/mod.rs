pub mod components;
pub mod systems;

use crate::commands::RpgCommand;
use crate::ecs::resources::{CountdownTimer, DeltaTime};
use crate::ecs::systems::assign_room_target::AssignRoomTargetSystem;
use crate::ecs::systems::combat::CombatSystem;
use crate::ecs::systems::command_handler::{CommandHandlerSystem, CommandQueue};
use crate::ecs::systems::countdown::CountdownSystem;
use crate::ecs::systems::death_handler::DeathCleanupSystem;
use crate::ecs::systems::item_pickup::ItemPickup;
use crate::ecs::systems::movement::Movement;
use crate::ecs::systems::movement_validation::MovementValidationSystem;
use crate::ecs::systems::pathfinding::PathfindingSystem;
use crate::ecs::systems::player_ai::PlayerAI;
use crate::ecs::systems::player_spacing::PartySpacing;
use crate::ecs::systems::random_wander::RandomWander;
use crate::ecs::systems::rendering::Rendering;
use crate::ecs::systems::shop_population::ShopPopulation;
use crate::ecs::world::create_world;
use common::GameSnapShot;
use specs::{Builder, DispatcherBuilder, Join, World, WorldExt};
use tokio::sync::broadcast;
use tokio::sync::mpsc::Receiver;
use crate::ecs::systems::room_exploration::RoomExplorationSystem;

pub mod resources;
mod shop;
mod world;

pub struct GameWorld {
    pub ecs: World,
    pub rx: Receiver<(String, RpgCommand, bool)>,
}

impl GameWorld {
    pub fn new(rx: Receiver<(String, RpgCommand, bool)>) -> Self {
        let mut world = create_world();
        Self { ecs: world, rx }
    }
}

pub fn run_game_server(
    gamestate_sender: broadcast::Sender<GameSnapShot>,
    commands_receiver: Receiver<(String, RpgCommand, bool)>,
) {
    const MIN_PLAYERS: usize = 1;

    let mut gw = GameWorld::new(commands_receiver);

    gw.ecs.insert(CommandQueue::default());

    // build dispatcher with systems
    let mut dispatcher = DispatcherBuilder::new()
        .with(CommandHandlerSystem, "command_handler", &[])
        .with(
            Rendering {
                sender: gamestate_sender,
            },
            "rendering",
            &[],
        )
        .with(
            ShopPopulation,
            "shop_population",
            &["rendering", "command_handler"],
        )
        .with(
            CountdownSystem {
                min_players: MIN_PLAYERS,
            },
            "countdown",
            &["command_handler"],
        )
        // .with(
        //     AssignRoomTargetSystem,
        //     "assign_room_target",
        //     &["command_handler"],
        // )
        // .with(RandomWander, "idle", &["movement"])

        // .with(PathfindingSystem, "pathfinding", &[])
        // .with(
        //     MovementValidationSystem,
        //     "movement_validation",
        //     &["pathfinding"],
        // )
        .with(Movement, "movement", &[])
        .with(PlayerAI, "player_ai", &[])
        .with(CombatSystem, "combat", &[])
        .with(DeathCleanupSystem, "death_cleanup", &[])
        .with(PartySpacing, "player_spacing", &["movement"])
        .with(ItemPickup, "item_pickup", &["movement"])
        .with(RoomExplorationSystem, "room_exploration", &["movement"])
        .build();

    let mut last_frame_time = std::time::Instant::now();
    loop {
        while let Ok((player, command, is_privileged)) = gw.rx.try_recv() {
            if let Some(queue) = gw.ecs.get_mut::<CommandQueue>() {
                queue.push_back((player, command, is_privileged));
            }
        }

        // clock timing
        let now = std::time::Instant::now();
        let delta = now.duration_since(last_frame_time).as_secs_f64();
        last_frame_time = now;
        gw.ecs.write_resource::<DeltaTime>().0 = delta;

        // run systems
        dispatcher.dispatch(&mut gw.ecs);

        // cleanup etc
        gw.ecs.maintain();

        // TODO: figure out a time interval appropriate for this game
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
