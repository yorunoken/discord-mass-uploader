use dotenvy::dotenv;
use serenity::prelude::*;
use std::env;

mod command_trait;
mod commands;
mod event_handler;
mod options;
mod utils;

#[tokio::main]
async fn main() {
    // Load the environment variables
    dotenv().ok();

    let discord_token =
        env::var("TOKEN").expect("Expected DISCORD_TOKEN to be defined in environment.");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Get commands
    let commands = options::get_commands();

    // Build the Discord client, and pass in our event handler
    let mut client = Client::builder(discord_token, intents)
        .event_handler(event_handler::Handler { commands })
        .await
        .expect("Error creating client.");

    // Run the Discord client (runs the ready function)
    if let Err(reason) = client.start().await {
        println!("Error starting client: {:?}", reason);
    }
}
