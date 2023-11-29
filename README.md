# Discord Mass Uploader

DMU is a simple Discord bot written in djs v14 to upload massive files by dividing them into chunks, and sending their binary data into a thread which you can then download easily with the click of a button.

## Building the bot from source

To build the bot from source you can follow these steps:

- Download and install [nodejs](https://nodejs.org)

- Install `tsc`
  - `npm install -g typescript`

- clone the repository
  - `git clone https://github.com/YoruNoKen/discord-mass-uploader`

- install dependencies
  - `npm install`

- create the `downloads` and `media` dirs
  - `mkdir media && mkdir downloads`

- create a `.env` file and assign `token` a [discord token](https://discord.com/developers/) like this:
  - `token=DISCORD_TOKEN_HERE`

- build and run the ts code
  - `npm run build && npm start`

## Using the bot

- Place the files you want to upload to `./media`
  - `mv ./path/to/file ./media`

- use the `/upload` command of the bot
  - `/upload name:godot format:exe`

- after the bot opens up the thread and uploads the chunks, you can click on the green download button to download your file back

## Contact me 🤙

if you have any questions or just want to have someone to talk to, add me on discord (@yorunoken), or message me on twitter (@ken_yoru)
