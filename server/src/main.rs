mod commands;
mod ecs;
mod parser;
mod webserver;

use crate::commands::{PlayerCommand, RpgCommand};
use crate::ecs::run_game_server;
use crate::parser::get_command;
use crate::webserver::start_web_server;
use common::{GameSnapShot, MenuItem};
use dotenv::dotenv;
use tmi::{Badge, BadgeData, Privmsg};
use tokio::sync::{broadcast, mpsc};

use common::PlayerClass;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let (commands_sender, commands_receiver) = mpsc::channel::<(String, RpgCommand, bool)>(100);

    let (gamestate_sender, _gamestate_receiver) = broadcast::channel::<GameSnapShot>(100);

    _ = tokio::spawn(read_commands_from_chat(commands_sender));
    _ = tokio::spawn(start_web_server(gamestate_sender.clone()));
    run_game_server(gamestate_sender, commands_receiver);
}

fn is_privileged_user(msg: &Privmsg) -> bool {
    msg.badges().any(|b| match b {
        Badge::Subscriber(..) => true,
        Badge::Other(badge) => badge.name() == "founder" || badge.name() == "premium",
        _ => false,
    })
}

fn is_channel_owner(msg: &Privmsg) -> bool {
    msg.badges().any(|b| matches!(b, Badge::Broadcaster))
}

async fn read_commands_from_chat(tx: mpsc::Sender<(String, RpgCommand, bool)>) {
    // TODO: remove
    // simulate players joining
    // _ = tx.try_send((
    //     "ubruntu".to_string(),
    //     RpgCommand::Join(PlayerClass::Fighter),
    //     true,
    // ));
    // _ = tx.try_send((
    //     "Pixelmog".to_string(),
    //     RpgCommand::Join(PlayerClass::Wizard),
    //     true,
    // ));
    // _ = tx.try_send((
    //     "tester".to_string(),
    //     RpgCommand::Join(PlayerClass::Rogue),
    //     true,
    // ));
    // _ = tx.try_send(("ubruntu".to_string(), RpgCommand::Difficulty(3), true));

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
        if let Ok(msg) = irc_client.recv().await
            && let Ok(m) = msg.as_typed()
        {
            match m {
                tmi::Message::Privmsg(msg) => {
                    if let Some(command) = get_command(&mut msg.text()) {
                        _ = tx.try_send((
                            msg.sender().name().to_string(),
                            command,
                            is_privileged_user(&msg),
                        ));
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
