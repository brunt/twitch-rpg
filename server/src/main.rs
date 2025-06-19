mod commands;
mod parser;
mod player_class;

mod ecs;
mod webserver;

use crate::commands::RpgCommand;
use crate::parser::get_command;
use dotenv::dotenv;
use specs::{World, WorldExt};
use tmi::User;
use tokio::sync::{broadcast, mpsc};
use crate::ecs::{run_game_server, GameSnapShot, GameState};
use crate::webserver::start_web_server;

#[tokio::main]
async fn main() {
    dotenv().ok();


    let (commands_sender, commands_receiver) = mpsc::channel::<(String, RpgCommand)>(100);
    let (gamestate_sender, _gamestate_receiver) = broadcast::channel::<GameSnapShot>(100);

    
    _ = tokio::spawn(read_commands_from_chat(commands_sender));
    _ = tokio::spawn(start_web_server(gamestate_sender.clone()));

    run_game_server(gamestate_sender, commands_receiver).await;
}


async fn read_commands_from_chat(tx: mpsc::Sender<(String, RpgCommand)>) {
    let channel = format!(
        "#{}",
        std::env::var("CHANNEL_NAME").expect("missing CHANNEL_NAME env var")
    );
    let mut irc_client = tmi::Client::anonymous()
        .await
        .expect("failed to create client");
    irc_client
        .join(&channel)
        .await
        .expect("failed to join channel");

    loop {
        if let Ok(msg) = irc_client.recv().await {
            if let Ok(m) = msg.as_typed() {
                match m {
                    tmi::Message::Privmsg(msg) => {
                        if let Some(command) = get_command(&mut msg.text()) {
                            let _ = tx.send((msg.sender().name().to_string(), command)).await;
                            // match command {
                            //     RpgCommand::New(class) => {
                            //         // create player character with default values, store in persistence (player, class)
                            //     }
                            //     RpgCommand::Load => {
                            //         // load character from persistence (player)
                            //     }
                            //     // RpgCommand::Use(consumable) => {
                            //     //     // check if player has the consumable
                            //     // }
                            //     // RpgCommand::Buy(item) => {
                            //     //     // subtract player gold, player gets item
                            //     // }
                            //     _ => unimplemented!(),
                            // }
                        }
                    }
                    tmi::Message::Reconnect => {
                        _ = irc_client.reconnect().await;
                        _ = irc_client.join(&channel).await;
                    }
                    tmi::Message::Ping(ping) => {
                        _ = irc_client.pong(&ping).await;
                    }
                    _ => {}
                }
            }
        }
    }
}
