import { Client, ClientOptions, Collection } from "discord.js";

export class MyClient extends Client {
    slashCommands: Collection<any, any>;
    client: any;
  
    constructor(options: ClientOptions) {
      super(options);
      this.slashCommands = new Collection();
    }
  }