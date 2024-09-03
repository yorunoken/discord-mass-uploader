use serenity::{
    all::{CommandInteraction, Context},
    async_trait,
    builder::CreateCommand,
    Error,
};

#[async_trait]
pub trait Command {
    fn name(&self) -> &'static str;

    async fn run(&self, ctx: &Context, command: &CommandInteraction) -> Result<(), Error>;

    fn register(&self) -> CreateCommand;
}
