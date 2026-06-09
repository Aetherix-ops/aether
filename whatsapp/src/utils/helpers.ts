// =============================================================
//  aether-wa — src/utils/helpers.ts
// =============================================================

import { WASocket, proto } from "@whiskeysockets/baileys";

/**
 * Get text content from a message
 */
export function getMessageText(msg: proto.IWebMessageInfo): string {
  const m = msg.message;
  if (!m) return "";

  return (
    m.conversation ||
    m.extendedTextMessage?.text ||
    m.imageMessage?.caption ||
    m.videoMessage?.caption ||
    m.buttonsResponseMessage?.selectedButtonId ||
    m.listResponseMessage?.singleSelectReply?.selectedRowId ||
    ""
  );
}

/**
 * Get sender JID from message
 */
export function getSender(msg: proto.IWebMessageInfo): string {
  return msg.key.participant || msg.key.remoteJid || "";
}

/**
 * Check if message is from a group
 */
export function isGroup(msg: proto.IWebMessageInfo): boolean {
  return msg.key.remoteJid?.endsWith("@g.us") ?? false;
}

/**
 * Get chat JID
 */
export function getChatId(msg: proto.IWebMessageInfo): string {
  return msg.key.remoteJid || "";
}

/**
 * Reply to a message
 */
export async function reply(
  sock: WASocket,
  msg: proto.IWebMessageInfo,
  text: string
): Promise<void> {
  const jid = getChatId(msg);
  await sock.sendMessage(jid, {
    text,
    contextInfo: {
      stanzaId: msg.key.id,
      participant: getSender(msg),
      quotedMessage: msg.message ?? undefined,
    },
  });
}

/**
 * Send a message without quoting
 */
export async function send(
  sock: WASocket,
  jid: string,
  text: string
): Promise<void> {
  await sock.sendMessage(jid, { text });
}

/**
 * React to a message with an emoji
 */
export async function react(
  sock: WASocket,
  msg: proto.IWebMessageInfo,
  emoji: string
): Promise<void> {
  await sock.sendMessage(getChatId(msg), {
    react: {
      text: emoji,
      key: msg.key,
    },
  });
}

/**
 * Format bytes to human readable
 */
export function bytesToHuman(bytes: number): string {
  if (bytes >= 1_073_741_824) return `${(bytes / 1_073_741_824).toFixed(1)}GB`;
  if (bytes >= 1_048_576)     return `${(bytes / 1_048_576).toFixed(1)}MB`;
  if (bytes >= 1_024)         return `${(bytes / 1_024).toFixed(1)}KB`;
  return `${bytes}B`;
}

/**
 * Sleep for N milliseconds
 */
export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Format uptime
 */
export function formatUptime(ms: number): string {
  const s = Math.floor(ms / 1000);
  const m = Math.floor(s / 60);
  const h = Math.floor(m / 60);
  const d = Math.floor(h / 24);
  return `${d}d ${h % 24}h ${m % 60}m ${s % 60}s`;
}
