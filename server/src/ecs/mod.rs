pub mod components;
pub mod systems;

use crate::commands::RpgCommand;
use crate::ecs::resources::DeltaTime;
use crate::ecs::systems::assign_room_target::AssignRoomTargetSystem;
use crate::ecs::systems::attack_cooldown::AttackCooldownSystem;
use crate::ecs::systems::combat::CombatSystem;
use crate::ecs::systems::command_handler::{CommandHandlerSystem, CommandQueue};
use crate::ecs::systems::countdown::CountdownSystem;
use crate::ecs::systems::death_handler::DeathCleanupSystem;
use crate::ecs::systems::dungeon_complete::DungeonComplete;
use crate::ecs::systems::effect_expiration::EffectExpirationSystem;
use crate::ecs::systems::enemy_ai::EnemyAISystem;
use crate::ecs::systems::group_coordination::GroupCoordination;
use crate::ecs::systems::item_pickup::ItemPickup;
use crate::ecs::systems::level_up::LevelUpSystem;
use crate::ecs::systems::movement::Movement;
use crate::ecs::systems::party_wipe::PartyWipeSystem;
use crate::ecs::systems::pathfinding::PathfindingSystem;
use crate::ecs::systems::player_ai::PlayerAI;
use crate::ecs::systems::projectile_cleanup::ProjectileCleanupSystem;
use crate::ecs::systems::random_wander::RandomWander;
use crate::ecs::systems::rendering::Rendering;
use crate::ecs::systems::room_exploration::RoomExplorationSystem;
use crate::ecs::systems::shop_population::ShopPopulation;
use crate::ecs::systems::stat_aggregation::StatAggregation;
use crate::ecs::systems::stat_change::StatSyncSystem;
use crate::ecs::systems::weapon_classification::WeaponClassifierSystem;
use crate::ecs::world::create_world;
use common::Effect::StatChange;
use common::GameSnapShot;
use specs::{Builder, DispatcherBuilder, Join, World, WorldExt};
use tokio::sync::broadcast;
use tokio::sync::mpsc::Receiver;

pub mod assets;
pub mod resources;
mod shop;
mod spells;
mod world;

pub struct GameWorld {
    pub ecs: World,
    pub rx: Receiver<(String, RpgCommand, bool)>,
}

impl GameWorld {
    pub fn new(rx: Receiver<(String, RpgCommand, bool)>) -> Self {
        let world = create_world();
        Self { ecs: world, rx }
    }
}

pub fn run_game_server(
    gamestate_sender: broadcast::Sender<GameSnapShot>,
    commands_receiver: Receiver<(String, RpgCommand, bool)>,
) {
    const MIN_PLAYERS: usize = 2;

    let mut gw = GameWorld::new(commands_receiver);

    gw.ecs.insert(CommandQueue::default());

    // build dispatcher with systems
    let mut dispatcher = DispatcherBuilder::new()
        .with(CommandHandlerSystem, "command_handler", &[])
        .with(
            CountdownSystem {
                min_players: MIN_PLAYERS,
            },
            "countdown",
            &["command_handler"],
        )
        .with(CombatSystem, "combat", &[])
        .with(DeathCleanupSystem, "death_cleanup", &[])
        .with(PartyWipeSystem, "party_wipe", &["death_cleanup"])
        .with(AttackCooldownSystem, "attack_cooldown", &["combat"])
        .with(Movement, "movement", &["combat", "death_cleanup"])
        .with(
            AssignRoomTargetSystem,
            "assign_room_target",
            &["combat", "movement"],
        )
        .with(GroupCoordination, "group_coordination", &[])
        .with(
            RandomWander,
            "random_wander",
            &["movement", "group_coordination"],
        )
        .with(PlayerAI, "player_ai", &["group_coordination"])
        .with(PathfindingSystem, "pathfinding", &["movement", "player_ai"])
        .with(
            Rendering {
                sender: gamestate_sender,
            },
            "rendering",
            &["combat"],
        )
        .with(
            ShopPopulation,
            "shop_population",
            &["rendering", "command_handler"],
        )
        .with(ItemPickup, "item_pickup", &["movement"])
        .with(RoomExplorationSystem, "room_exploration", &["movement"])
        .with(EnemyAISystem, "enemy_ai", &["movement"])
        // .with(PartySpacing, "player_spacing", &["movement", "player_ai"])
        .with(
            ProjectileCleanupSystem,
            "projectile_cleanup",
            &["rendering"],
        )
        .with(DungeonComplete, "dungeon_complete", &[])
        .with(LevelUpSystem, "level_up", &["dungeon_complete"])
        .with(WeaponClassifierSystem, "weapon_classification", &[])
        .with(EffectExpirationSystem, "effect_expiration", &[])
        .with(StatAggregation, "stat_aggregation", &[])
        .with(
            StatSyncSystem,
            "stat_sync",
            &["effect_expiration", "stat_aggregation"],
        )
        .build();

    let mut last_frame_time = std::time::Instant::now();
    let target_frame_duration = std::time::Duration::from_millis(500);
    loop {
        let loop_start = std::time::Instant::now();

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
        dispatcher.dispatch(&gw.ecs);
        // slow_dispatcher.dispatch(&gw.ecs);

        // cleanup etc
        gw.ecs.maintain();

        let elapsed = loop_start.elapsed();
        if elapsed < target_frame_duration {
            std::thread::sleep(target_frame_duration - elapsed);
        }
    }
}
