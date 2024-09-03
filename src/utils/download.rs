use std::{fs::File, io::Write, path::Path};

use base64::{engine::general_purpose, Engine as _};
use reqwest;
use serenity::all::{ChannelId, ComponentInteraction, Context, EditInteractionResponse};

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

        loop {
            let messages = ctx
                .http
                .get_messages(ChannelId::new(thread_id), None, Some(100))
                .await
                .unwrap();

            if messages.is_empty() {
                break;
            }

            all_messages.extend(messages.clone());

            if messages.len() < 100 {
                break;
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
        let mut file_content = String::new();

        for (index, message) in all_messages.iter().enumerate() {
            if let Some(attachment) = message.attachments.first() {
                component
                    .edit_response(
                        &ctx.http,
                        EditInteractionResponse::new().content(format!(
                            "Downloading content... ({}/{})",
                            index + 1,
                            all_messages.len(),
                        )),
                    )
                    .await
                    .unwrap();

                let response = reqwest::get(&attachment.url).await.unwrap();
                let chunk_content = response.text().await.unwrap();
                file_content.push_str(&chunk_content);
            }
        }

        let decoded_content = general_purpose::STANDARD.decode(file_content).unwrap();

        let file_path = Path::new("downloads").join(format!("{}.{}", name, format));
        let mut file = File::create(&file_path).unwrap();
        file.write_all(&decoded_content).unwrap();

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
