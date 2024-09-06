use api::AppState;
use dotenvy::dotenv;
use serenity::prelude::*;
use std::env;

mod api;
mod database;
mod event_handler;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    // Load the environment variables
    dotenv().unwrap();

    let discord_token =
        env::var("TOKEN").expect("Expected DISCORD_TOKEN to be defined in environment.");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Build the Discord client, and pass in our event handler
    let mut client = Client::builder(discord_token, intents)
        .event_handler(event_handler::Handler)
        .await
        .expect("Error creating client.");

    let http = client.http.clone();

    let client_future = client.start();

    let pool = database::create_pool().await;
    let state = AppState::new();
    let api = routes::routes(pool, http.clone(), state);

    let port = 8000;
    println!("Listening on http://localhost:{}", port);

    tokio::select! {
        _ = client_future => println!("Client ended unexpectedly"),
        _ = warp::serve(api).run(([127, 0, 0, 1], port)) => println!("API server ended unexpectedly"),
    }
}
