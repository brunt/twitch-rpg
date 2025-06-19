use serde::Serialize;
use specs::{Dispatcher, World, WorldExt};
use specs_derive::Component;
use tokio::sync::{broadcast, mpsc};
use crate::commands::RpgCommand;
use crate::ecs::components::{Position, Renderable};

mod components;
mod systems;

mod entities;

pub struct GameState {
    ecs: World,
    rx: mpsc::Receiver<(String, RpgCommand)>,
    tx: broadcast::Sender<GameSnapShot>,
    // dispatcher: Dispatcher<'static, 'static>,
}

impl GameState{
    pub fn new(rx: mpsc::Receiver<(String, RpgCommand)>, tx: broadcast::Sender<GameSnapShot>) -> Self {
        let mut ecs = World::new();
        ecs.register::<Position>();
        // ecs.register::<Renderable>();
        // ecs.register::<Health>();

        Self {
            ecs,
            rx,
            tx
        }
    }
}



#[derive(Clone, Serialize)]
pub struct GameSnapShot {

}

pub async fn run_game_server(gamestate_sender: mpsc::Sender<GameSnapShot>, commands_receiver: mpsc::Receiver<(String, RpgCommand)>) {
    let gs = GameSate::new()
}