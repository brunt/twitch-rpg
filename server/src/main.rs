mod commands;
mod ecs;
mod parser;
mod player_class;
mod webserver;

use crate::commands::RpgCommand;
use crate::ecs::{GameSnapShot, run_game_server};
use crate::parser::get_command;
use crate::webserver::start_web_server;
use dotenv::dotenv;
use tmi::{Badge, Privmsg};
use tokio::sync::{broadcast, mpsc};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let (commands_sender, commands_receiver) = mpsc::channel::<(String, RpgCommand, bool)>(100);
    
    // TODO: broadcast or mpsc??
    let (gamestate_sender, _gamestate_receiver) = broadcast::channel::<GameSnapShot>(100);

    _ = tokio::spawn(read_commands_from_chat(commands_sender));
    _ = tokio::spawn(start_web_server(gamestate_sender.clone()));
    run_game_server(gamestate_sender, commands_receiver);
}

fn is_privileged_user(msg: &Privmsg) -> bool {
    msg.badges().any(|b| matches!(b, Badge::Subscriber(..)))
}

fn is_channel_owner(msg: &Privmsg) -> bool {
    msg.badges().any(|b| matches!(b, Badge::Broadcaster))
}

async fn read_commands_from_chat(tx: mpsc::Sender<(String, RpgCommand, bool)>) {
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
                            dbg!(&command);
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
}
