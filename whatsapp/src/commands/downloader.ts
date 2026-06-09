// =============================================================
//  aether-wa — src/commands/downloader.ts
//  Downloader: yt, ytv, tiktok, ig
// =============================================================

import axios from "axios";
import { CommandContext } from "../types/context";
import { reply } from "../utils/helpers";

export async function handle(cmd: string, ctx: CommandContext): Promise<boolean> {
  switch (cmd) {
    case "yt":      await youtube(ctx, "audio"); return true;
    case "ytv":     await youtube(ctx, "video"); return true;
    case "tiktok":  await tiktok(ctx); return true;
    case "ig":      await instagram(ctx); return true;
    default: return false;
  }
}

async function youtube(ctx: CommandContext, type: "audio" | "video"): Promise<void> {
  const url = ctx.args[0];
  if (!url || !url.includes("youtube.com") && !url.includes("youtu.be")) {
    await reply(ctx.sock, ctx.msg,
      `❌ Usage: ${ctx.config.prefix}${type === "audio" ? "yt" : "ytv"} <youtube_url>`
    );
    return;
  }

  await reply(ctx.sock, ctx.msg, `⏳ Downloading YouTube ${type}...`);

  try {
    // Using public ytdl API
    const apiUrl = `https://api.vevioz.com/api/button/${type === "audio" ? "mp3" : "mp4"}?url=${encodeURIComponent(url)}`;

    await reply(ctx.sock, ctx.msg,
      `✅ *YouTube ${type === "audio" ? "Audio" : "Video"}*\n\nDownload link:\n${apiUrl}\n\n_Note: Link expires in 24 hours_`
    );
  } catch (e: any) {
    await reply(ctx.sock, ctx.msg, `❌ Download failed: ${e.message}`);
  }
}

async function tiktok(ctx: CommandContext): Promise<void> {
  const url = ctx.args[0];
  if (!url || !url.includes("tiktok.com")) {
    await reply(ctx.sock, ctx.msg, `❌ Usage: ${ctx.config.prefix}tiktok <tiktok_url>`);
    return;
  }

  await reply(ctx.sock, ctx.msg, "⏳ Downloading TikTok video...");

  try {
    const res = await axios.get(
      `https://api.tiklydown.eu.org/api/download/v3?url=${encodeURIComponent(url)}`,
      { timeout: 15000 }
    );

    const data = res.data;
    if (!data.video?.noWatermark) {
      await reply(ctx.sock, ctx.msg, "❌ Could not fetch TikTok video.");
      return;
    }

    const videoRes = await axios.get(data.video.noWatermark, {
      responseType: "arraybuffer",
      timeout: 30000,
    });

    await ctx.sock.sendMessage(ctx.chatId, {
      video: Buffer.from(videoRes.data),
      caption: `🎵 ${data.title || "TikTok Video"}\n👤 ${data.author?.name || "Unknown"}`,
    });
  } catch (e: any) {
    await reply(ctx.sock, ctx.msg, `❌ Download failed: ${e.message}`);
  }
}

async function instagram(ctx: CommandContext): Promise<void> {
  const url = ctx.args[0];
  if (!url || !url.includes("instagram.com")) {
    await reply(ctx.sock, ctx.msg, `❌ Usage: ${ctx.config.prefix}ig <instagram_url>`);
    return;
  }

  await reply(ctx.sock, ctx.msg, "⏳ Fetching Instagram media...");

  try {
    const res = await axios.get(
      `https://api.instagramdl.eu.org/?url=${encodeURIComponent(url)}`,
      { timeout: 15000 }
    );

    const media = res.data?.data?.[0];
    if (!media) {
      await reply(ctx.sock, ctx.msg, "❌ Could not fetch Instagram media.");
      return;
    }

    const mediaRes = await axios.get(media.url, {
      responseType: "arraybuffer",
      timeout: 30000,
    });

    const buffer = Buffer.from(mediaRes.data);
    const isVideo = media.type === "video" || media.url.includes(".mp4");

    if (isVideo) {
      await ctx.sock.sendMessage(ctx.chatId, {
        video: buffer,
        caption: "📸 Instagram Video",
      });
    } else {
      await ctx.sock.sendMessage(ctx.chatId, {
        image: buffer,
        caption: "📸 Instagram Image",
      });
    }
  } catch (e: any) {
    await reply(ctx.sock, ctx.msg, `❌ Download failed: ${e.message}`);
  }
}
