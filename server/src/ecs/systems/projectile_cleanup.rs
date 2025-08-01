use crate::ecs::components::combat::FiredProjectile;
use specs::{Entities, Join, System, WriteStorage}; // Adjust path to your FiredProjectile component

/// This system removes the FiredProjectile component from entities
/// after it has been processed by the rendering system.
pub struct ProjectileCleanupSystem;

impl<'a> System<'a> for ProjectileCleanupSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, FiredProjectile>);

    fn run(&mut self, (entities, mut fired_projectiles): Self::SystemData) {
        // Iterate over all entities that have a FiredProjectile component
        let mut to_remove = vec![];
        for (entity, _) in (&entities, &fired_projectiles).join() {
            to_remove.push(entity);
        }
        for entity in to_remove {
            fired_projectiles.remove(entity);
        }
    }
}
