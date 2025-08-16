use crate::ecs::components::Enemy;
use crate::ecs::components::Experience;
use crate::ecs::components::Level;
use crate::ecs::components::Money;
use crate::ecs::components::Name;
use crate::ecs::components::Player;
use crate::ecs::components::Position;
use crate::ecs::components::Stats;
use crate::ecs::components::class::{CharacterClass, ShowCharacter};
use crate::ecs::components::combat::{
    ActionTimer, AttackComponent, AttackTarget, DefenseComponent, FiredProjectile, HealthComponent,
    MeleeAttacker, RangedAttacker,
};
use crate::ecs::components::effect::ActiveEffects;
use crate::ecs::components::form::FormComponent;
use crate::ecs::components::inventory::Equipment;
use crate::ecs::components::movement::{
    CanMove, DesiredTargetPosition, MovementSpeed, TargetPosition, Wander,
};
use crate::ecs::components::spells::{SpellCaster, SpellTarget, SpellTimer, Spellbook};
use crate::ecs::components::{DungeonItem, Opened};
use crate::ecs::resources::{
    Adventure, CountdownTimer, DeltaTime, DungeonLoot, GameState, GroupDestination, ShopInventory,
};
use crate::ecs::shop::ShopItemPool;
use crate::ecs::spells::AllSpells;
use specs::{World, WorldExt};

pub fn create_world() -> World {
    let mut world = World::new();
    world.register::<Name>();
    world.register::<Position>();
    world.register::<MovementSpeed>();
    world.register::<TargetPosition>();
    world.register::<DesiredTargetPosition>();
    world.register::<CharacterClass>();
    world.register::<HealthComponent>();
    world.register::<Equipment>();
    world.register::<Stats>();
    world.register::<Experience>();
    world.register::<Player>();
    world.register::<Level>();
    world.register::<Money>();
    world.register::<Enemy>();
    world.register::<DungeonItem>();
    world.register::<Wander>();
    world.register::<AttackTarget>();
    world.register::<AttackComponent>();
    world.register::<MeleeAttacker>();
    world.register::<RangedAttacker>();
    world.register::<DefenseComponent>();
    world.register::<Opened>();
    world.register::<ShowCharacter>();
    world.register::<FiredProjectile>();
    world.register::<CanMove>();
    world.register::<ActionTimer>();
    world.register::<SpellTarget>();
    world.register::<SpellTimer>();
    world.register::<SpellCaster>();
    world.register::<FormComponent>();
    world.register::<ActiveEffects>();
    world.register::<Spellbook>();

    // resources
    world.insert(GameState::InTown);
    world.insert::<Option<CountdownTimer>>(None);
    world.insert(DeltaTime::default());
    world.insert(ShopItemPool::new());
    world.insert(GroupDestination {
        target_room_id: None,
    });
    world.insert(ShopInventory::default());
    world.insert(Option::<Adventure>::None);
    world.insert(Option::<DungeonLoot>::None);
    world.insert(AllSpells::new());
    world
}
