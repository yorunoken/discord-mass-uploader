use base64::{engine::general_purpose, Engine as _};
use std::fs::{read, File};

use serenity::{
    all::{
        CommandInteraction, CommandOptionType, CreateActionRow, CreateAttachment, CreateButton,
        CreateCommandOption, CreateEmbed, CreateMessage, CreateThread, EditInteractionResponse,
        Error,
    },
    async_trait,
    builder::CreateCommand,
    prelude::*,
};

use crate::command_trait::Command;

const CHUNK_SIZE: usize = 251_658_24; // 24 MB

pub struct Upload;

#[async_trait]
impl Command for Upload {
    fn name(&self) -> &'static str {
        "upload"
    }

    async fn run(&self, ctx: &Context, command: &CommandInteraction) -> Result<(), Error> {
        command.defer(&ctx.http).await?;

        let file_option = command.data.options.iter().find(|opt| opt.name == "file");
        let format_option = command.data.options.iter().find(|opt| opt.name == "format");

        let file = match file_option {
            Some(option) => match &option.value {
                serenity::all::CommandDataOptionValue::String(value) => value,
                _ => return Err(Error::Other("Invalid file option type")),
            },
            None => return Err(Error::Other("File option not provided")),
        };

        let file_format = match format_option {
            Some(option) => match &option.value {
                serenity::all::CommandDataOptionValue::String(value) => value,
                _ => return Err(Error::Other("Invalid file format type")),
            },
            None => return Err(Error::Other("File format not provided")),
        };

        if let Err(_) = File::open(format!("media/{}.{}", file, file_format)) {
            command
                .edit_response(
                    &ctx.http,
                    EditInteractionResponse::new().content(format!(
                        "The file `{}.{}` does not exist in the `media` folder.",
                        file, file_format
                    )),
                )
                .await?;

            return Ok(());
        }

        command
            .edit_response(
                &ctx.http,
                EditInteractionResponse::new().content("Opening thread..."),
            )
            .await?;

        let thread = command
            .channel_id
            .create_thread(
                &ctx.http,
                CreateThread::new(format!("{}.{}", file, file_format))
                    .kind(serenity::all::ChannelType::PublicThread),
            )
            .await?;

        command
            .edit_response(
                &ctx.http,
                EditInteractionResponse::new().content("Reading file..."),
            )
            .await?;

        let file_base64 = match read(format!("media/{}.{}", file, file_format)) {
            Ok(content) => general_purpose::STANDARD.encode(&content),
            Err(_) => {
                command
                    .edit_response(
                        &ctx.http,
                        EditInteractionResponse::new().content("Error reading file."),
                    )
                    .await?;

                return Ok(());
            }
        };
        let total_chunks = (file_base64.len() as f64 / CHUNK_SIZE as f64).ceil() as usize;

        command
            .edit_response(
                &ctx.http,
                EditInteractionResponse::new().content("Uploading file..."),
            )
            .await?;

        for i in 0..total_chunks {
            let start = i * CHUNK_SIZE;
            let end = std::cmp::min((i + 1) * CHUNK_SIZE, file_base64.len());
            let chunk = &file_base64[start..end];

            let attachment =
                CreateAttachment::bytes(chunk.as_bytes(), format!("{}_{}.txt", file, i));

            thread
                .send_message(
                    &ctx.http,
                    CreateMessage::new()
                        .content(format!("{}_{}.txt", file, i))
                        .add_file(attachment),
                )
                .await?;

            command
                .edit_response(
                    &ctx.http,
                    EditInteractionResponse::new().content(format!(
                        "Uploading file... ({}/{})",
                        i + 1,
                        total_chunks,
                    )),
                )
                .await?;
        }

        let download_button = CreateButton::new("download")
            .label("Download")
            .style(serenity::all::ButtonStyle::Success);

        let action_row = CreateActionRow::Buttons(vec![download_button]);

        let builder = EditInteractionResponse::new()
            .content(format!("The file `{}.{}` has been uploaded. You can download the file to your `downloads` folder by clicking on the green download button.", file, file_format))
            .embed(CreateEmbed::new().title("File information").fields(vec![("Name", file, false), ("Format", file_format, false), ("Thread ID", &thread.id.to_string() , false)]))
            .components(vec![action_row]);
        command.edit_response(&ctx.http, builder).await?;

        Ok(())
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Upload files")
            .set_options(vec![
                CreateCommandOption::new(CommandOptionType::String, "file", "The file name")
                    .required(true),
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "format",
                    "The format of your file (eg. exe, mp4, png, zip)",
                )
                .required(true),
            ])
    }
}
