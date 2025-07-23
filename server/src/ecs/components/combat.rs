use crate::ecs::components::NullStorage;
use specs::Entity;
use common::{Health, PlayerClass};
use crate::ecs::components::Component;
use crate::ecs::components::DenseVecStorage;


/// the entity that this entity is attacking
#[derive(Component)]
pub struct AttackTarget {
    pub(crate) entity: Entity
}

#[derive(Component)]
pub struct DefenseComponent {
    /// the entity's rating of flat damage reduction
    pub defense: u32,
    /// the entity's rating of how often hits against them do no damage
    pub evasion: u32,
}

#[derive(Component)]
pub struct AttackComponent {
    /// base damage that this entity can do
    pub damage: u32,
    /// calculates chance of dealing damage (or critical strike) with each hit
    pub hit_rating: u32,
    /// the range in tiles that is how far this entity can attack another entity
    pub range: u32,
    /// the time after which this entity may attack again
    pub cooldown: u32, // milliseconds?
}

#[derive(Component)]
#[storage(NullStorage)]
pub struct MeleeAttacker;

#[derive(Component)]
#[storage(NullStorage)]
pub struct RangedAttacker;

#[derive(Debug, Component)]
pub struct HealthComponent(pub Health);

impl Default for HealthComponent {
    fn default() -> Self {
        Self(Health::Alive { hp: 1, max_hp: 1 })
    }
}

impl HealthComponent {
    pub fn is_alive(&self) -> bool {
        match self.0 {
            Health::Alive { .. } => true,
            _ => false,
        }
    }

    pub fn new_from_class(class: &PlayerClass) -> Self {
        Self(match class {
            PlayerClass::Fighter | PlayerClass::Paladin | PlayerClass::Ranger => {
                Health::Alive { hp: 10, max_hp: 10 }
            }
            PlayerClass::Rogue
            | PlayerClass::Cleric
            | PlayerClass::Druid
            | PlayerClass::Warlock => Health::Alive { hp: 8, max_hp: 8 },
            PlayerClass::Wizard | PlayerClass::Sorcerer => Health::Alive { hp: 6, max_hp: 6 },
        })
    }
}