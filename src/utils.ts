import { ButtonInteraction, Message, TextChannel } from "discord.js";
import fs from "fs";

export async function downloadFiles(interaction: ButtonInteraction, message: Message<boolean>) {
  if (!interaction.guild || !(interaction.channel instanceof TextChannel)) {
    return;
  }

  await interaction.reply("Fetching thread information..");
  const description = message.embeds[0].description.split("\n");

  const thread = (await interaction.guild.channels.fetch(
    description
      .find((item) => item.includes("Thread ID"))
      .split(":")[1]
      .trim(),
  )) as TextChannel;
  if (!thread || !thread.messages) {
    return interaction.editReply(`Couldn't find the thread of this file.`);
  }

  interaction.editReply(`Fetching messages.`);
  const unsortedMessages = [];
  while (true) {
    const msg = await thread.messages.fetch({ limit: 100 });
    unsortedMessages.push(...msg);
    if (Object.entries(msg).length < 100) break;
  }

  await interaction.editReply(`Downloading files..`);
  download(
    interaction,
    unsortedMessages
      .toSorted((a, b) => Number(b[0]) - Number(a[0]))
      .map((msg) => msg[1])
      .flatMap((item) => item.attachments.map((attachment) => attachment.url))
      .reverse(),
    description
      .find((item) => item.includes("Name"))
      .split(":")[1]
      .trim(),
    description
      .find((item) => item.includes("Type"))
      .split(":")[1]
      .trim(),
  );
}

async function download(interaction: ButtonInteraction, urls: string[], name: string, format: string) {
  const binaryParts = await Promise.all(urls.map((url) => fetch(url).then((response) => response.arrayBuffer())));

  fs.writeFileSync(
    `./downloads/${name}.${format}`,
    new Uint8Array(binaryParts.reduce((acc, part) => [...acc, ...new Uint8Array(part)], [])),
  );
  await interaction.editReply(`Your file has been downloaded to: ./downloads/${name}.${format}`);
}
