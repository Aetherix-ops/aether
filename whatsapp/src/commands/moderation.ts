// =============================================================
//  aether-wa — src/commands/moderation.ts
//  Group moderation: kick, add, promote, demote, everyone,
//  antilink, mute, unmute
// =============================================================

import { CommandContext } from "../types/context";
import { reply, send } from "../utils/helpers";

export async function handle(cmd: string, ctx: CommandContext): Promise<boolean> {
  switch (cmd) {
    case "kick":     await kick(ctx); return true;
    case "add":      await add(ctx); return true;
    case "promote":  await promote(ctx); return true;
    case "demote":   await demote(ctx); return true;
    case "everyone": await everyone(ctx); return true;
    case "antilink": await antilink(ctx); return true;
    case "mute":     await mute(ctx); return true;
    case "unmute":   await unmute(ctx); return true;
    default: return false;
  }
}

function requireGroup(ctx: CommandContext): boolean {
  if (!ctx.group) {
    reply(ctx.sock, ctx.msg, "❌ This command can only be used in a group.");
    return false;
  }
  return true;
}

function getMentioned(ctx: CommandContext): string[] {
  const msg = ctx.msg.message;
  return (
    msg?.extendedTextMessage?.contextInfo?.mentionedJid ||
    msg?.buttonsResponseMessage?.contextInfo?.mentionedJid ||
    []
  );
}

async function kick(ctx: CommandContext): Promise<void> {
  if (!requireGroup(ctx)) return;

  const mentioned = getMentioned(ctx);
  if (mentioned.length === 0) {
    await reply(ctx.sock, ctx.msg, "❌ Please mention a user to kick.");
    return;
  }

  try {
    await ctx.sock.groupParticipantsUpdate(ctx.chatId, mentioned, "remove");
    await reply(ctx.sock, ctx.msg, `✅ Kicked ${mentioned.length} member(s).`);
  } catch {
    await reply(ctx.sock, ctx.msg, "❌ Failed to kick. Make sure I am an admin.");
  }
}

async function add(ctx: CommandContext): Promise<void> {
  if (!requireGroup(ctx)) return;

  const number = ctx.args[0]?.replace(/[^0-9]/g, "");
  if (!number) {
    await reply(ctx.sock, ctx.msg, `❌ Usage: ${ctx.config.prefix}add 628xxxxxxxx`);
    return;
  }

  const jid = `${number}@s.whatsapp.net`;

  try {
    await ctx.sock.groupParticipantsUpdate(ctx.chatId, [jid], "add");
    await reply(ctx.sock, ctx.msg, `✅ Added ${number} to the group.`);
  } catch {
    await reply(ctx.sock, ctx.msg, "❌ Failed to add member.");
  }
}

async function promote(ctx: CommandContext): Promise<void> {
  if (!requireGroup(ctx)) return;

  const mentioned = getMentioned(ctx);
  if (mentioned.length === 0) {
    await reply(ctx.sock, ctx.msg, "❌ Please mention a user to promote.");
    return;
  }

  try {
    await ctx.sock.groupParticipantsUpdate(ctx.chatId, mentioned, "promote");
    await reply(ctx.sock, ctx.msg, `✅ Promoted ${mentioned.length} member(s) to admin.`);
  } catch {
    await reply(ctx.sock, ctx.msg, "❌ Failed to promote.");
  }
}

async function demote(ctx: CommandContext): Promise<void> {
  if (!requireGroup(ctx)) return;

  const mentioned = getMentioned(ctx);
  if (mentioned.length === 0) {
    await reply(ctx.sock, ctx.msg, "❌ Please mention a user to demote.");
    return;
  }

  try {
    await ctx.sock.groupParticipantsUpdate(ctx.chatId, mentioned, "demote");
    await reply(ctx.sock, ctx.msg, `✅ Demoted ${mentioned.length} admin(s).`);
  } catch {
    await reply(ctx.sock, ctx.msg, "❌ Failed to demote.");
  }
}

async function everyone(ctx: CommandContext): Promise<void> {
  if (!requireGroup(ctx)) return;

  try {
    const meta = await ctx.sock.groupMetadata(ctx.chatId);
    const members = meta.participants.map((p) => p.id);
    const mentions = members.join(" ");
    const text = ctx.args.join(" ") || "📢 Attention everyone!";

    await ctx.sock.sendMessage(ctx.chatId, {
      text: `@${members.map((m) => m.split("@")[0]).join(" @")}\n\n${text}`,
      mentions: members,
    });
  } catch {
    await reply(ctx.sock, ctx.msg, "❌ Failed to tag everyone.");
  }
}

const antilinkGroups = new Set<string>();

async function antilink(ctx: CommandContext): Promise<void> {
  if (!requireGroup(ctx)) return;

  const action = ctx.args[0]?.toLowerCase();

  if (action === "on") {
    antilinkGroups.add(ctx.chatId);
    await reply(ctx.sock, ctx.msg, "✅ Anti-link enabled for this group.");
  } else if (action === "off") {
    antilinkGroups.delete(ctx.chatId);
    await reply(ctx.sock, ctx.msg, "✅ Anti-link disabled for this group.");
  } else {
    const status = antilinkGroups.has(ctx.chatId) ? "ON" : "OFF";
    await reply(ctx.sock, ctx.msg, `Anti-link is currently *${status}*\nUse: ${ctx.config.prefix}antilink on/off`);
  }
}

async function mute(ctx: CommandContext): Promise<void> {
  if (!requireGroup(ctx)) return;

  try {
    await ctx.sock.groupSettingUpdate(ctx.chatId, "announcement");
    await reply(ctx.sock, ctx.msg, "🔇 Group muted. Only admins can send messages.");
  } catch {
    await reply(ctx.sock, ctx.msg, "❌ Failed to mute group.");
  }
}

async function unmute(ctx: CommandContext): Promise<void> {
  if (!requireGroup(ctx)) return;

  try {
    await ctx.sock.groupSettingUpdate(ctx.chatId, "not_announcement");
    await reply(ctx.sock, ctx.msg, "🔊 Group unmuted. All members can send messages.");
  } catch {
    await reply(ctx.sock, ctx.msg, "❌ Failed to unmute group.");
  }
}
