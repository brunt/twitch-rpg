use crate::ecs::components::class::CharacterClass;
use crate::ecs::components::{
    Equipment, Experience, Health, HealthComponent, Level, Money, MovementAI, MovementSpeed, Name,
    Player, Position, Projectile, Resource, Stats, TargetPosition,
};
use crate::ecs::resources::{CountdownTimer, DeltaTime, GameState, ShopInventory};
use specs::{World, WorldExt};
use crate::ecs::shop::{initialize_shop_items, ShopItemPool};

pub fn change_game_state(world: &mut World, new_state: GameState) {
    if let Some(state) = world.get_mut::<GameState>() {
        *state = new_state.clone();
        println!("Game state changed to: {:?}", new_state);
    }
}

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
    world
}
