use crate::ecs::components::class::CharacterClass;
use crate::ecs::components::{
    Experience, HealthComponent, Level, Money, MovementAI, MovementSpeed, Name,
    Player, Position, Projectile, Resource, Stats, TargetPosition,
};
use crate::ecs::resources::{CountdownTimer, DeltaTime, GameState, ShopInventory};
use specs::{World, WorldExt};
use crate::ecs::components::inventory::{Equipment};
use crate::ecs::shop::{initialize_shop_items, ShopItemPool};

pub fn create_world() -> World {
    let mut world = World::new();
    world.register::<Name>();
    world.register::<Position>();
    world.register::<MovementSpeed>();
    world.register::<TargetPosition>();
    world.register::<CharacterClass>();
    world.register::<HealthComponent>();
    world.register::<Equipment>();
    world.register::<Resource>();
    world.register::<Stats>();
    world.register::<Experience>();
    world.register::<MovementAI>();
    world.register::<Player>();
    world.register::<Projectile>();
    world.register::<Level>();
    world.register::<Money>();

    // resources
    world.insert(GameState::OutOfDungeon);
    world.insert::<Option<CountdownTimer>>(None);
    world.insert(DeltaTime::default());
    world.insert(ShopItemPool{ all_items: initialize_shop_items() });
    world.insert(ShopInventory::default());
    // world.insert(tatami_dungeon::Dungeon::generate_with_params(..GenerateDungeonParams::default()))
    world.insert(tatami_dungeon::Dungeon::generate());
    world
}
