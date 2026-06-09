// =============================================================
//  aether-wa — src/commands/general.ts
//  General: help, ping, info, uptime, owner
// =============================================================

import { CommandContext } from "../types/context";
import { reply } from "../utils/helpers";
import { formatUptime } from "../utils/helpers";

export async function handle(cmd: string, ctx: CommandContext): Promise<boolean> {
  switch (cmd) {
    case "help":    await help(ctx); return true;
    case "ping":    await ping(ctx); return true;
    case "info":    await info(ctx); return true;
    case "uptime":  await uptime(ctx); return true;
    case "owner":   await owner(ctx); return true;
    default: return false;
  }
}

async function help(ctx: CommandContext): Promise<void> {
  const p = ctx.config.prefix;
  const text = `
╔══════════════════════════════╗
║    *AETHER BOT — COMMANDS*    ║
╚══════════════════════════════╝

*General*
${p}help — Command list
${p}ping — Check latency
${p}info — Bot information
${p}uptime — Bot uptime
${p}owner — Owner info

*Moderation* _(group admin only)_
${p}kick @user — Kick member
${p}add 628xx — Add member
${p}promote @user — Promote to admin
${p}demote @user — Demote admin
${p}everyone — Tag all members
${p}antilink on/off — Anti-link toggle
${p}mute — Mute group
${p}unmute — Unmute group

*Utility*
${p}sticker — Convert image to sticker
${p}toimg — Convert sticker to image
${p}qr <text> — Generate QR code
${p}calc <expr> — Calculator
${p}weather <city> — Weather info

*Downloader*
${p}yt <url> — YouTube audio
${p}ytv <url> — YouTube video
${p}tiktok <url> — TikTok video
${p}ig <url> — Instagram media

*AI*
${p}ai <message> — Chat with AI
${p}imagine <prompt> — Generate image

*Pterodactyl*
${p}ptero list — List servers
${p}ptero status <id> — Server status
${p}ptero start <id> — Start server
${p}ptero stop <id> — Stop server
${p}ptero restart <id> — Restart server
`.trim();

  await reply(ctx.sock, ctx.msg, text);
}

async function ping(ctx: CommandContext): Promise<void> {
  const start = Date.now();
  await reply(ctx.sock, ctx.msg, "🏓 Pinging...");
  const latency = Date.now() - start;
  await reply(ctx.sock, ctx.msg, `🏓 *Pong!*\n> Latency: *${latency}ms*`);
}

async function info(ctx: CommandContext): Promise<void> {
  const text = `
╔══════════════════════════════╗
║      *⚡ AETHER BOT*          ║
╚══════════════════════════════╝

*Version* : 1.0.0
*Language* : TypeScript
*Library*  : Baileys
*Prefix*   : ${ctx.config.prefix}
*Uptime*   : ${formatUptime(Date.now() - ctx.startTime)}
*Author*   : Aetherix-ops

_github.com/Aetherix-ops/aether_
`.trim();

  await reply(ctx.sock, ctx.msg, text);
}

async function uptime(ctx: CommandContext): Promise<void> {
  const up = formatUptime(Date.now() - ctx.startTime);
  await reply(ctx.sock, ctx.msg, `⏱️ *Uptime:* ${up}`);
}

async function owner(ctx: CommandContext): Promise<void> {
  const ownerNum = ctx.config.ownerNumber;
  if (!ownerNum) {
    await reply(ctx.sock, ctx.msg, "Owner not configured.");
    return;
  }
  await reply(ctx.sock, ctx.msg, `👑 *Owner:* wa.me/${ownerNum}`);
}
