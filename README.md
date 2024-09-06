# Discord Mass Uploader

Discord Mass Uploader is a powerful tool that allows users to upload large files to Discord servers and download them later. It breaks down files into smaller chunks, stores them in Discord threads, and reassembles them upon download.

## Tech Stack

### Frontend
- Next.js
- TypeScript
- Tailwind CSS
- shadcn/ui components

### Backend
- Rust
- Warp web framework
- SQLite database
- Serenity Discord library

## Setup

### Prerequisites
- Node.js (or Bun) and npm
- Rust and Cargo
- Discord Bot Token

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yorunoken/discord-mass-uploader.git
   cd discord-mass-uploader
   ```

2. Set up root:
    ```
    npm install
    ```

3. Configure environment variables:

   For the backend:
   - Navigate to the `backend` folder
   - Rename `.env.example` to `.env`
   - Open `.env` and fill in your Discord bot token:
   ```
   TOKEN=your_discord_bot_token_here
   PORT=8000
   DATABASE_URL=sqlite://data.db
   ```

   For the frontend:
   - Navigate to the `frontend` folder
   - Rename `.env.example` to `.env`
   - The default configuration should work, but you can modify if needed:
   ```
   BACKEND_PORT=8000
    ```

4. Build frontend and backend:
    ```
    npm run build
    ```

## Usage

1. Start the server:
   ```
   npm run start
   ```

3. Open your browser and navigate to `http://localhost:3000`

4. To upload a file:
   - Click on "Upload" from the main page
   - Enter your Discord channel ID
   - Select a file to upload
   - Click "Start Upload"

5. To download a file:
   - Click on "Download" from the main page
   - Find the file you want to download
   - Click the "Download" button next to the file

## Important Notes

- Ensure your Discord bot has the necessary permissions in the target channel.
- Files are downloaded to your default Downloads folder.
- Be cautious when downloading, as files with the same name will be overwritten.
- There's a small chance Discord might delete your files, so please do not upload any sensitive information here.
- I'm not responsible for any damages caused by this application, use at your own risk.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
