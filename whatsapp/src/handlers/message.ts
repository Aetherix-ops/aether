// =============================================================
//  aether-wa — src/handlers/message.ts
//  Main message handler — routes to command plugins
// =============================================================

import { WASocket, proto } from "@whiskeysockets/baileys";
import { loadConfig } from "../config";
import { getMessageText, getSender, isGroup, getChatId, reply, react } from "../utils/helpers";
import { logger } from "../utils/logger";

// Import command plugins
import * as general from "../commands/general";
import * as moderation from "../commands/moderation";
import * as pterodactyl from "../commands/pterodactyl";
import * as ai from "../commands/ai";
import * as downloader from "../commands/downloader";
import * as utility from "../commands/utility";

const startTime = Date.now();
const config = loadConfig();

export async function handleMessage(
  sock: WASocket,
  msg: proto.IWebMessageInfo
): Promise<void> {
  const text = getMessageText(msg).trim();
  const chatId = getChatId(msg);
  const sender = getSender(msg);
  const group = isGroup(msg);

  if (!text) return;

  const prefix = config.prefix;

  // ── AUTO REPLY ───────────────────────────────────
  if (config.features.autoReply && !text.startsWith(prefix)) {
    await handleAutoReply(sock, msg, text);
    return;
  }

  // ── PREFIX COMMANDS ──────────────────────────────────
  if (!text.startsWith(prefix)) return;

  const [rawCmd, ...args] = text.slice(prefix.length).trim().split(/\s+/);
  const cmd = rawCmd.toLowerCase();

  logger.cmd(`${sender} → ${prefix}${cmd} ${args.join(" ")}`);

  // React with clock while processing
  await react(sock, msg, "⏳");

  try {
    const ctx = { sock, msg, args, chatId, sender, group, config, startTime };

    // Route to command
    const handled =
      (await general.handle(cmd, ctx)) ||
      (await moderation.handle(cmd, ctx)) ||
      (await utility.handle(cmd, ctx)) ||
      (await downloader.handle(cmd, ctx)) ||
      (config.pterodactyl.enabled && (await pterodactyl.handle(cmd, ctx))) ||
      (config.features.aiEnabled && (await ai.handle(cmd, ctx)));

    if (!handled) {
      await react(sock, msg, "❌");
      await reply(sock, msg, `Command *${prefix}${cmd}* not found.\nType *${prefix}help* for the command list.`);
      return;
    }

    // Success react
    await react(sock, msg, "✅");

  } catch (e) {
    logger.error(`Command error [${cmd}]:`, e);
    await react(sock, msg, "❌");
    await reply(sock, msg, `❌ An error occurred while processing your command.`);
  }
}

async function handleAutoReply(
  sock: WASocket,
  msg: proto.IWebMessageInfo,
  text: string
): Promise<void> {
  const lower = text.toLowerCase();

  const replies: Record<string, string> = {
    "hi": "Hello! 👋 How can you help you?",
    "hello": "Hey there! 👋",
    "halo": "Halo! 👋 Ada yang bisa dibantu?",
    "ping": "Pong! 🏓",
  };

  for (const [trigger, response] of Object.entries(replies)) {
    if (lower === trigger || lower.startsWith(trigger + " ")) {
      await reply(sock, msg, response);
      return;
    }
  }
}
