use crate::{command_trait::Command, utils::download::download_file};
use serenity::{
    async_trait,
    model::{application::Command as ApplicationCommand, prelude::*},
    prelude::*,
};

pub struct Handler {
    pub commands: Vec<Box<dyn Command + Send + Sync>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                for cmd in &self.commands {
                    if cmd.name() == command.data.name.as_str() {
                        if let Err(why) = cmd.run(&ctx, &command).await {
                            println!("Error executing slash command: {:?}", why);
                        }
                    }
                }
            }

            Interaction::Component(component) => download_file(component, ctx).await,

            _ => {}
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands = self
            .commands
            .iter()
            .map(|c| c.register())
            .collect::<Vec<_>>();

        ApplicationCommand::set_global_commands(&ctx.http, commands)
            .await
            .expect("Failed to set global application commands");
    }
}
