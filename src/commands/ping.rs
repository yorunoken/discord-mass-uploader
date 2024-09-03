use serenity::{
    all::{CommandInteraction, Error},
    async_trait,
    builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage},
    prelude::*,
};

use crate::command_trait::Command;

pub struct Ping;

#[async_trait]
impl Command for Ping {
    fn name(&self) -> &'static str {
        "ping"
    }

    async fn run(&self, ctx: &Context, command: &CommandInteraction) -> Result<(), Error> {
        let content = "Pong!";

        let builder = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(content),
        );
        command.create_response(&ctx.http, builder).await?;

        Ok(())
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description("Check if bot is alive")
    }
}
