# Discord Mass Uploader

A Discord bot for uploading and downloading large files in chunks.

## How to Use

1. Download the appropriate binary for your system from the [releases section](https://github.com/yorunoken/discord-mass-uploader/releases) of this repository.

2. Create two folders in the same directory as the binary:
   - `media`: Place files you want to upload here.
   - `downloads`: Downloaded files will be saved here.

3. Create a `.env` file in the same directory as the binary with the following content:
   ```
   TOKEN=your_discord_bot_token_here
   ```
   Replace `your_discord_bot_token_here` with your actual Discord bot token.

4. Run the binary to start the bot.

5. In Discord, use the following commands:
   - `/upload file:[filename] format:[file_extension]`: Uploads a file from the `media` folder.
   - `/ping`: Checks if the bot is responsive.

6. To download a file, click the "Download" button that appears after a successful upload.

**Note:** Ensure your Discord bot has the necessary permissions in your server to send messages, create threads, and use slash commands.
