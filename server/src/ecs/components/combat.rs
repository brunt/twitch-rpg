use crate::ecs::components::Component;
use crate::ecs::components::DenseVecStorage;

#[derive(Component)]
pub struct AttackRange {
    pub range: u32,
}

pub struct TargetEntity {
    
}

pub struct Evasion{
    pub evasion: f64,
}

pub struct AttackComponent {
    
}

pub struct AttackCooldown{
    pub cooldown: u32, // milliseconds
}