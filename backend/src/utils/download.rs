use base64::{engine::general_purpose, Engine as _};
use dirs;
use reqwest;
use serenity::all::{ChannelId, Http};
use std::sync::Arc;
use tokio::{io::AsyncWriteExt, sync::mpsc::Sender};

pub async fn download_file(thread_id: String, name: String, http: Arc<Http>, tx: Sender<f32>) {
    let thread_id: u64 = thread_id.parse().expect("Failed to parse thread ID");
    let mut all_messages = Vec::new();
    let mut last_message_id = None;

    loop {
        let messages = http
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

    // Reverse the vec
    all_messages.reverse();

    let download_dir = dirs::download_dir().expect("Failed to get Downloads directory");
    let file_path = download_dir.join(name);
    let mut file = tokio::fs::File::create(&file_path).await.unwrap();

    let total_messages = all_messages.len();
    let mut processed_messages = 0;

    for message in all_messages {
        if let Some(attachment) = message.attachments.first() {
            let response = reqwest::get(&attachment.url).await.unwrap();
            let chunk_content = response.text().await.unwrap();

            let decoded_chunk = general_purpose::STANDARD.decode(chunk_content).unwrap();
            file.write_all(&decoded_chunk).await.unwrap();

            processed_messages += 1;
            let progress = processed_messages as f32 / total_messages as f32 * 100.0;
            tx.send(progress).await.unwrap();
        }
    }

    file.flush().await.unwrap();
    tx.send(100.0).await.unwrap();
}
