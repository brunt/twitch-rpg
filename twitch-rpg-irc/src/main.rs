mod commands;
mod parser;

use crate::commands::RpgCommand;
use crate::parser::get_command;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let channel = format!(
        "#{}",
        std::env::var("CHANNEL_NAME").expect("missing CHANNEL_NAME env var")
    );

    let action_url = format!(
        "http://localhost:{}/action",
        std::env::var("PORT").unwrap_or("9091".to_string())
    );

    let http_client = reqwest::Client::new();
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
                            let player = msg.sender().name();
                            match command {
                                RpgCommand::New(class) => {
                                    // create player character with default values, store in persistance
                                }
                                RpgCommand::Buy(item) => {
                                    // subtract player gold, player gets item
                                }
                                RpgCommand::Load => {
                                    // load character from persistance
                                }
                                RpgCommand::Use(consumable) => {
                                    // check if player has the consumable
                                }
                            }
                        }

                        //TODO: in these match branches do a req like this
                        // if let Some(command) = get_command(&mut msg.text()) {
                        //     _ = http_client
                        //         .post(&action_url)
                        //         .json(&ActionRequest::from(command))
                        //         .send()
                        //         .await;
                        // }
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
