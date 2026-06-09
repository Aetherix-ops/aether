// =============================================================
//  aether-wa — src/commands/ai.ts
//  AI chat using OpenRouter API
// =============================================================

import axios from "axios";
import { CommandContext } from "../types/context";
import { reply } from "../utils/helpers";

export async function handle(cmd: string, ctx: CommandContext): Promise<boolean> {
  switch (cmd) {
    case "ai":
    case "chat":
      await aiChat(ctx);
      return true;
    default:
      return false;
  }
}

async function aiChat(ctx: CommandContext): Promise<void> {
  const prompt = ctx.args.join(" ");

  if (!prompt) {
    await reply(ctx.sock, ctx.msg, `❌ Usage: ${ctx.config.prefix}ai <message>`);
    return;
  }

  const apiKey = ctx.config.features.openrouterKey;
  if (!apiKey) {
    await reply(ctx.sock, ctx.msg, "❌ AI is not configured. Set OPENROUTER_KEY in config.");
    return;
  }

  try {
    const res = await axios.post(
      "https://openrouter.ai/api/v1/chat/completions",
      {
        model: "openai/gpt-4o-mini",
        messages: [
          {
            role: "system",
            content: `You are Aether, a helpful WhatsApp bot assistant. Be concise and friendly. Bot name: ${ctx.config.botName}.`,
          },
          { role: "user", content: prompt },
        ],
        max_tokens: 500,
      },
      {
        headers: {
          Authorization: `Bearer ${apiKey}`,
          "Content-Type": "application/json",
          "HTTP-Referer": "https://github.com/Aetherix-ops/aether",
        },
        timeout: 30000,
      }
    );

    const answer = res.data.choices?.[0]?.message?.content || "No response.";
    await reply(ctx.sock, ctx.msg, `🤖 *Aether AI*\n\n${answer}`);
  } catch (e: any) {
    await reply(ctx.sock, ctx.msg, `❌ AI error: ${e.message}`);
  }
}
  
