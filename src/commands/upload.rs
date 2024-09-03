use base64::{engine::general_purpose, Engine as _};
use indicatif::{ProgressBar, ProgressStyle};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};

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

        let file_name = match file_option {
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

        let file_path = format!("media/{}.{}", file_name, file_format);
        let file = match File::open(&file_path).await {
            Ok(file) => file,
            Err(_) => {
                command
                    .edit_response(
                        &ctx.http,
                        EditInteractionResponse::new().content(format!(
                            "The file `{}` does not exist in the `media` folder.",
                            file_path
                        )),
                    )
                    .await?;
                return Ok(());
            }
        };

        let file_size = file.metadata().await?.len();
        let mut reader = BufReader::new(file);

        let thread = command
            .channel_id
            .create_thread(
                &ctx.http,
                CreateThread::new(format!("{}.{}", file_name, file_format))
                    .kind(serenity::all::ChannelType::PublicThread),
            )
            .await?;

        let pb = ProgressBar::new(file_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );

        let mut buffer = vec![0; CHUNK_SIZE];
        let mut chunk_index = 0;

        loop {
            let bytes_read = reader.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }

            let chunk = &buffer[..bytes_read];
            let base64_chunk = general_purpose::STANDARD.encode(chunk);

            let attachment = CreateAttachment::bytes(
                base64_chunk.as_bytes(),
                format!("{}_{}.txt", file_name, chunk_index),
            );

            thread
                .send_message(
                    &ctx.http,
                    CreateMessage::new()
                        .content(format!("{}_{}.txt", file_name, chunk_index))
                        .add_file(attachment),
                )
                .await?;

            pb.inc(bytes_read as u64);
            chunk_index += 1;

            if chunk_index % 10 == 0 {
                command
                    .edit_response(
                        &ctx.http,
                        EditInteractionResponse::new().content(format!(
                            "Uploading file... ({:.2}%)",
                            (pb.position() as f64 / file_size as f64) * 100.0
                        )),
                    )
                    .await?;
            }
        }

        pb.finish_with_message("Upload complete");

        let download_button = CreateButton::new("download")
            .label("Download")
            .style(serenity::all::ButtonStyle::Success);

        let action_row = CreateActionRow::Buttons(vec![download_button]);

        let builder = EditInteractionResponse::new()
            .content(format!("The file `{}.{}` has been uploaded. You can download the file to your `downloads` folder by clicking on the green download button.", file_name, file_format))
            .embed(CreateEmbed::new().title("File information").fields(vec![("Name", file_name, false), ("Format", file_format, false), ("Thread ID", &thread.id.to_string() , false)]))
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
