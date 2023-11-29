import { Interaction, InteractionType } from "discord.js";
import { MyClient } from "../classes";
import { downloadFiles } from "../utils";

export const name = "interactionCreate";
export const execute = async (interaction: Interaction, client: MyClient) => {
  if (!interaction.guildId) {
    return;
  }

  if (interaction.type === InteractionType.MessageComponent && interaction.isButton()) {
    downloadFiles(interaction, interaction.message);
  }

  if (interaction.type === InteractionType.ApplicationCommand) {
    try {
      const command = client.slashCommands.get(interaction.commandName);
      command.run({ client, interaction });
    } catch (e) {
      console.error(e);
      interaction.reply({ content: "There was an error with this interaction. Please try again.", ephemeral: true });
    }
  }
};
