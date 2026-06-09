// =============================================================
//  aether-wa — src/commands/utility.ts
//  Utility: sticker, toimg, qr, calc, weather
// =============================================================

import { CommandContext } from "../types/context";
import { reply } from "../utils/helpers";

export async function handle(cmd: string, ctx: CommandContext): Promise<boolean> {
  switch (cmd) {
    case "sticker": await sticker(ctx); return true;
    case "toimg":   await toimg(ctx); return true;
    case "qr":      await qr(ctx); return true;
    case "calc":    await calc(ctx); return true;
    case "weather": await weather(ctx); return true;
    default: return false;
  }
}

async function sticker(ctx: CommandContext): Promise<void> {
  const quoted = ctx.msg.message?.extendedTextMessage?.contextInfo?.quotedMessage;
  const imageMsg = quoted?.imageMessage || ctx.msg.message?.imageMessage;

  if (!imageMsg) {
    await reply(ctx.sock, ctx.msg, `❌ Please send or quote an image with ${ctx.config.prefix}sticker`);
    return;
  }

  try {
    const buffer = await ctx.sock.downloadMediaMessage(
      { message: { imageMessage: imageMsg }, key: ctx.msg.key } as any
    );

    await ctx.sock.sendMessage(ctx.chatId, {
      sticker: buffer as Buffer,
    });
  } catch {
    await reply(ctx.sock, ctx.msg, "❌ Failed to create sticker.");
  }
}

async function toimg(ctx: CommandContext): Promise<void> {
  const quoted = ctx.msg.message?.extendedTextMessage?.contextInfo?.quotedMessage;
  const stickerMsg = quoted?.stickerMessage || ctx.msg.message?.stickerMessage;

  if (!stickerMsg) {
    await reply(ctx.sock, ctx.msg, `❌ Please quote a sticker with ${ctx.config.prefix}toimg`);
    return;
  }

  try {
    const buffer = await ctx.sock.downloadMediaMessage(
      { message: { stickerMessage: stickerMsg }, key: ctx.msg.key } as any
    );

    await ctx.sock.sendMessage(ctx.chatId, {
      image: buffer as Buffer,
      caption: "Here's your image! 🖼️",
    });
  } catch {
    await reply(ctx.sock, ctx.msg, "❌ Failed to convert sticker to image.");
  }
}

async function qr(ctx: CommandContext): Promise<void> {
  const text = ctx.args.join(" ");
  if (!text) {
    await reply(ctx.sock, ctx.msg, `❌ Usage: ${ctx.config.prefix}qr <text>`);
    return;
  }

  try {
    const QRCode = await import("qrcode");
    const buffer = await QRCode.toBuffer(text, {
      width: 512,
      margin: 2,
      color: { dark: "#000000", light: "#ffffff" },
    });

    await ctx.sock.sendMessage(ctx.chatId, {
      image: buffer,
      caption: `QR Code for: *${text}*`,
    });
  } catch {
    await reply(ctx.sock, ctx.msg, "❌ Failed to generate QR code.");
  }
}

async function calc(ctx: CommandContext): Promise<void> {
  const expr = ctx.args.join(" ");
  if (!expr) {
    await reply(ctx.sock, ctx.msg, `❌ Usage: ${ctx.config.prefix}calc <expression>`);
    return;
  }

  // Sanitize: only allow safe math chars
  if (!/^[0-9+\-*/.() %]+$/.test(expr)) {
    await reply(ctx.sock, ctx.msg, "❌ Invalid expression. Only basic math operators allowed.");
    return;
  }

  try {
    const result = Function(`"use strict"; return (${expr})`)();
    await reply(ctx.sock, ctx.msg, `🧮 *${expr}* = *${result}*`);
  } catch {
    await reply(ctx.sock, ctx.msg, "❌ Invalid math expression.");
  }
}

async function weather(ctx: CommandContext): Promise<void> {
  const city = ctx.args.join(" ");
  if (!city) {
    await reply(ctx.sock, ctx.msg, `❌ Usage: ${ctx.config.prefix}weather <city>`);
    return;
  }

  try {
    const axios = (await import("axios")).default;
    const res = await axios.get(
      `https://wttr.in/${encodeURIComponent(city)}?format=3`,
      { timeout: 8000 }
    );

    await reply(ctx.sock, ctx.msg, `🌤️ *Weather*\n${res.data}`);
  } catch {
    await reply(ctx.sock, ctx.msg, "❌ Could not fetch weather data.");
  }
}
