use base64::{engine::general_purpose, Engine as _};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use serenity::all::{ChannelId, ComponentInteraction, Context, EditInteractionResponse};
use std::path::Path;
use tokio::io::AsyncWriteExt;

pub async fn download_file(component: ComponentInteraction, ctx: Context) {
    component.defer_ephemeral(&ctx.http).await.unwrap();

    let message = &component.message;

    component
        .edit_response(
            &ctx.http,
            EditInteractionResponse::new().content("Extracting content from embed..."),
        )
        .await
        .unwrap();

    if let Some(embed) = message.embeds.first() {
        let mut thread_id_str = "";
        let mut name = "";
        let mut format = "";

        for field in embed.fields.iter() {
            match field.name.as_str() {
                "Name" => name = &field.value,
                "Format" => format = &field.value,
                "Thread ID" => thread_id_str = &field.value,
                _ => {}
            }
        }

        component
            .edit_response(
                &ctx.http,
                EditInteractionResponse::new().content("Getting messages..."),
            )
            .await
            .unwrap();

        let thread_id: u64 = thread_id_str.parse().expect("Failed to parse thread ID");
        let mut all_messages = Vec::new();
        let mut last_message_id = None;

        loop {
            let messages = ctx
                .http
                .get_messages(
                    ChannelId::new(thread_id),
                    last_message_id.map(|id| serenity::all::MessagePagination::Before(id)),
                    Some(100),
                )
                .await
                .unwrap();

            if messages.is_empty() {
                break;
            }

            all_messages.extend(messages.clone());

            if messages.len() < 100 {
                break;
            }

            if let Some(last_message) = messages.last() {
                last_message_id = Some(last_message.id);
            }
        }

        component
            .edit_response(
                &ctx.http,
                EditInteractionResponse::new().content("Downloading content..."),
            )
            .await
            .unwrap();

        // Reverse the vec
        all_messages.reverse();

        let file_path = Path::new("downloads").join(format!("{}.{}", name, format));
        let mut file = tokio::fs::File::create(&file_path).await.unwrap();

        let total_messages = all_messages.len();
        let pb = ProgressBar::new(total_messages as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );

        for (index, message) in all_messages.iter().enumerate() {
            if let Some(attachment) = message.attachments.first() {
                let response = reqwest::get(&attachment.url).await.unwrap();
                let chunk_content = response.text().await.unwrap();

                let decoded_chunk = general_purpose::STANDARD.decode(chunk_content).unwrap();
                file.write_all(&decoded_chunk).await.unwrap();

                pb.inc(1);

                if index % 10 == 0 || index == total_messages - 1 {
                    component
                        .edit_response(
                            &ctx.http,
                            EditInteractionResponse::new().content(format!(
                                "Downloading content... ({}/{})",
                                index + 1,
                                total_messages,
                            )),
                        )
                        .await
                        .unwrap();
                }
            }
        }

        pb.finish_with_message("Download complete");
        file.flush().await.unwrap();

        component
            .edit_response(
                &ctx.http,
                EditInteractionResponse::new().content(format!(
                    "File downloaded successfully to {}",
                    file_path.display()
                )),
            )
            .await
            .unwrap();
    }
}
