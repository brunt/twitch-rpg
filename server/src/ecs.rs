use specs::{Dispatcher, World};
use specs_derive::Component;

mod components;
mod systems;

mod entities;

#[derive(Serialize)]
pub struct GameState{
    ecs: World,
    #[serde(skip)]
    dispatcher: Dispatcher<'static, 'static>,
}

impl GameState{
    pub fn snapshot(&self) -> serde_json::Result<> {
        
    }
}