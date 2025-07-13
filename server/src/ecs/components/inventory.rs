use crate::ecs::components::Component;
use crate::ecs::components::DenseVecStorage;
use common::ShopItem;



#[derive(Component)]
pub struct Inventory {
    pub items: Vec<ShopItem>, //TODO: different type? components make sense for players
}