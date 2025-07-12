use crate::ecs::components::class::CharacterClass;
use crate::ecs::components::{
    Equipment, Experience, Health, HealthComponent, Level, Money, MovementAI, MovementSpeed, Name,
    Player, Position, Projectile, Resource, Stats, TargetPosition,
};
use crate::ecs::resources;
use crate::ecs::resources::{CountdownTimer, DeltaTime, GameState, ShopInventory};
use specs::{Entity, World, WorldExt};

pub fn change_game_state(world: &mut World, new_state: resources::GameState) {
    if let Some(state) = world.get_mut::<resources::GameState>() {
        *state = new_state.clone();
        println!("Game state changed to: {:?}", new_state);
    }
}

/// Registers a player in the town, allowing them to use the shop
// pub fn register_town_player(world: &mut World, player_name: String, entity: Entity) {
//     if let Some(town) = world.get_mut::<TownPlayers>() {
//         town.players.insert(player_name, entity);
//     }
// }
// pub fn register_town_player(
//     town_players: &mut WriteExpect<TownPlayers>,
//     player_name: String,
//     entity: Entity
// ) {
//     town_players.players.insert(player_name, entity);
// }

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
    world
}
