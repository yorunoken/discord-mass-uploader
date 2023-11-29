import { GatewayIntentBits } from "discord.js";
import fs from "fs";
import { MyClient } from "./classes";

const client = new MyClient({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMembers,
    GatewayIntentBits.GuildPresences,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.DirectMessages,
    GatewayIntentBits.MessageContent,
  ],
});

fs.readdirSync("./src/handlers").forEach((file: any) => {
  const event = require(`./handlers/${file.split(".")[0]}`);
  client.on(event.name, (...args: any) => event.execute(...args, client));
});

process.on("unhandledRejection", (e) => {
  console.error(e);
});
process.on("uncaughtException", (e) => {
  console.error(e);
});
process.on("uncaughtExceptionMonitor", (e) => {
  console.error(e);
});

client.login(process.env.token);
