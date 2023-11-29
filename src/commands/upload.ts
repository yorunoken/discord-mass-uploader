import {
  ActionRowBuilder,
  ButtonBuilder,
  ButtonStyle,
  ChatInputCommandInteraction,
  ComponentBuilder,
  EmbedBuilder,
  SlashCommandBuilder,
  TextChannel,
} from "discord.js";
import fs from "fs";
import stream from "stream";

export async function run({ interaction }: { interaction: ChatInputCommandInteraction }) {
  await interaction.deferReply();
  if (!interaction.guild || !(interaction.channel instanceof TextChannel)) {
    return;
  }

  const fileName = interaction.options.getString("name");
  const fileFormat = interaction.options.getString("format");
  const filePath = `./media/${fileName}.${fileFormat}`;
  if (!fs.existsSync) {
    return interaction.editReply(`The file ${fileName}.${fileFormat} doesn't exist in ./media`);
  }

  const threadExists = interaction.channel.threads.cache.find((thread) => thread.name === `${fileName}.${fileFormat}`);
  if (threadExists) {
    return interaction.editReply(
      `The thread with name ${fileName}.${fileFormat} already exists. Please try changing the name of the file.`,
    );
  }

  await interaction.editReply("Opening thread...");
  const thread = await interaction.channel.threads.create({
    name: `${fileName}.${fileFormat}`,
    autoArchiveDuration: 60,
  });

  await interaction.editReply("Reading file...");
  const chunkSize = 24 * 1024 * 1024; // 24 MB
  const fileData = await fs.promises.readFile(filePath, "base64");
  const totalChunks = Math.ceil(fileData.length / chunkSize);

  await interaction.editReply("Converting file to binary...");
  for (let i = 0; i < totalChunks; i++) {
    const start = i * chunkSize;
    const end = Math.min((i + 1) * chunkSize, fileData.length);
    const chunk = fileData.slice(start, end);

    const bufferStream = new stream.PassThrough();
    bufferStream.end(Buffer.from(chunk, "base64"));

    await thread.send({ files: [{ attachment: bufferStream, name: `${fileName}_${i}.txt` }] });
    await interaction.editReply(
      `Uploaded chunk ${i + 1} of ${totalChunks} (${(((i + 1) / totalChunks) * 100).toFixed(2)}%)`,
    );
  }

  await interaction.editReply({
    content:
      `The file "${fileName}.${fileFormat}" has been uploaded. You can download the file to your computer by clicking on the green "Download" button.`,
    embeds: [
      new EmbedBuilder().setTitle(`File information`).setDescription(
        `Name: ${fileName}\nType: ${fileFormat}\nSize: ${
          (fileData.length / 1024 / 1024).toFixed(2)
        }Mb\nThread ID: ${thread.id}`,
      ),
    ],
    components: [
      new ActionRowBuilder().addComponents(
        new ButtonBuilder().setCustomId("download").setStyle(ButtonStyle.Success).setLabel("Download"),
      ) as any,
    ],
  });
}

export const data = new SlashCommandBuilder()
  .setName("upload")
  .setDescription("Mass upload files to discord")
  .addStringOption((o) => o.setName("name").setDescription("The file name.").setRequired(true))
  .addStringOption((o) => o.setName("format").setDescription("The format of your file").setRequired(true));
