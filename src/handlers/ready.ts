import { REST } from "@discordjs/rest";
import { Routes } from "discord-api-types/v10";
import fs from "fs";
import { MyClient } from "../classes";
import { token } from "../constants";

const rest = new REST({ version: "10" }).setToken(token);
async function loadSlashCommands(client: MyClient) {
  if (!client.user) return;

  const slashCommands: any = [];
  const commands = fs.readdirSync("./src/commands");
  for (const cmd of commands) {
    const command = require(`../commands/${cmd.split(".")[0]}`);
    slashCommands.push(command.data.toJSON());

    client.slashCommands.set(command.data.name, command);
  }

  try {
    await rest.put(Routes.applicationCommands(client.user.id), { body: slashCommands });
  } catch (error) {
    console.error(error);
  }
}

export const name = "ready";
export const execute = async (_: any, client: MyClient) => {
  if (!client.user) return;

  console.log(`Logged in as ${client.user.tag}`);
  await loadSlashCommands(client);
  console.log("Loaded commands");
};
