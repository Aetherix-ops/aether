// =============================================================
//  aether-wa — src/config.ts
//  Configuration loader
// =============================================================

import fs from "fs";
import path from "path";

export interface Config {
  phoneNumber: string;
  prefix: string;
  ownerNumber: string;
  botName: string;
  pterodactyl: {
    enabled: boolean;
    panelUrl: string;
    apiKey: string;
  };
  features: {
    autoReply: boolean;
    welcomeMessage: boolean;
    antiLink: boolean;
    aiEnabled: boolean;
    openrouterKey: string;
  };
}

const defaultConfig: Config = {
  phoneNumber: process.env.PHONE_NUMBER || "",
  prefix: process.env.PREFIX || "!",
  ownerNumber: process.env.OWNER_NUMBER || "",
  botName: process.env.BOT_NAME || "Aether",
  pterodactyl: {
    enabled: process.env.PTERO_ENABLED === "true",
    panelUrl: process.env.PTERO_URL || "",
    apiKey: process.env.PTERO_API_KEY || "",
  },
  features: {
    autoReply: process.env.AUTO_REPLY === "true",
    welcomeMessage: process.env.WELCOME_MESSAGE === "true",
    antiLink: process.env.ANTI_LINK === "true",
    aiEnabled: process.env.AI_ENABLED === "true",
    openrouterKey: process.env.OPENROUTER_KEY || "",
  },
};

export function loadConfig(): Config {
  const configPath = path.join(process.cwd(), "config.json");

  if (fs.existsSync(configPath)) {
    try {
      const raw = fs.readFileSync(configPath, "utf-8");
      const json = JSON.parse(raw);
      return { ...defaultConfig, ...json };
    } catch {
      console.warn("Invalid config.json, using environment variables.");
    }
  }

  return defaultConfig;
  }
  
