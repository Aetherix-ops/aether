// =============================================================
//  aether-wa — src/commands/pterodactyl.ts
//  Pterodactyl: list, status, start, stop, restart
// =============================================================

import axios from "axios";
import { CommandContext } from "../types/context";
import { reply } from "../utils/helpers";
import { bytesToHuman } from "../utils/helpers";

async function apiGet(url: string, key: string, endpoint: string) {
  const res = await axios.get(`${url}/api/client${endpoint}`, {
    headers: {
      Authorization: `Bearer ${key}`,
      Accept: "application/json",
    },
    timeout: 10000,
  });
  return res.data;
}

async function apiPost(url: string, key: string, endpoint: string, body: object) {
  await axios.post(`${url}/api/client${endpoint}`, body, {
    headers: {
      Authorization: `Bearer ${key}`,
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    timeout: 10000,
  });
}

export async function handle(cmd: string, ctx: CommandContext): Promise<boolean> {
  if (cmd !== "ptero") return false;

  const sub = ctx.args[0]?.toLowerCase();
  const id  = ctx.args[1];

  const { panelUrl, apiKey } = ctx.config.pterodactyl;

  if (!panelUrl || !apiKey) {
    await reply(ctx.sock, ctx.msg, "❌ Pterodactyl is not configured.");
    return true;
  }

  switch (sub) {
    case "list":    await pteroList(ctx, panelUrl, apiKey); break;
    case "status":  await pteroStatus(ctx, panelUrl, apiKey, id); break;
    case "start":   await pteroPower(ctx, panelUrl, apiKey, id, "start"); break;
    case "stop":    await pteroPower(ctx, panelUrl, apiKey, id, "stop"); break;
    case "restart": await pteroPower(ctx, panelUrl, apiKey, id, "restart"); break;
    default:
      await reply(ctx.sock, ctx.msg,
        `❌ Unknown subcommand.\n\nUsage:\n${ctx.config.prefix}ptero list\n${ctx.config.prefix}ptero status <id>\n${ctx.config.prefix}ptero start/stop/restart <id>`
      );
  }

  return true;
}

async function pteroList(ctx: CommandContext, url: string, key: string): Promise<void> {
  try {
    const data = await apiGet(url, key, "/");
    const servers = data.data || [];

    if (servers.length === 0) {
      await reply(ctx.sock, ctx.msg, "No servers found.");
      return;
    }

    const lines = servers.map((s: any) => {
      const a = s.attributes;
      return `▸ *${a.name}*\n  ID: \`${a.identifier}\`\n  Node: ${a.node || "?"}`;
    });

    await reply(ctx.sock, ctx.msg,
      `╔══════════════════════╗\n║  *PTERODACTYL SERVERS*  ║\n╚══════════════════════╝\n\n${lines.join("\n\n")}\n\n_Total: ${servers.length} servers_`
    );
  } catch (e: any) {
    await reply(ctx.sock, ctx.msg, `❌ API error: ${e.message}`);
  }
}

async function pteroStatus(ctx: CommandContext, url: string, key: string, id?: string): Promise<void> {
  if (!id) {
    await reply(ctx.sock, ctx.msg, `❌ Usage: ${ctx.config.prefix}ptero status <id>`);
    return;
  }

  try {
    const data = await apiGet(url, key, `/servers/${id}/resources`);
    const attr = data.attributes;
    const res  = attr.resources;

    const stateIcon: Record<string, string> = {
      running:  "🟢",
      offline:  "🔴",
      starting: "🟡",
      stopping: "🟠",
    };

    const icon = stateIcon[attr.current_state] || "⚪";

    await reply(ctx.sock, ctx.msg,
      `╔══════════════════════╗\n║   *SERVER STATUS*       ║\n╚══════════════════════╝\n\n*ID*     : \`${id}\`\n*Status* : ${icon} ${attr.current_state}\n*RAM*    : ${bytesToHuman(res.memory_bytes || 0)}\n*CPU*    : ${(res.cpu_absolute || 0).toFixed(1)}%\n*Disk*   : ${bytesToHuman(res.disk_bytes || 0)}`
    );
  } catch (e: any) {
    await reply(ctx.sock, ctx.msg, `❌ API error: ${e.message}`);
  }
}

async function pteroPower(
  ctx: CommandContext,
  url: string,
  key: string,
  id?: string,
  signal?: string
): Promise<void> {
  if (!id) {
    await reply(ctx.sock, ctx.msg, `❌ Usage: ${ctx.config.prefix}ptero ${signal} <id>`);
    return;
  }

  const icons: Record<string, string> = {
    start: "▶️", stop: "⏹️", restart: "🔄"
  };

  try {
    await apiPost(url, key, `/servers/${id}/power`, { signal });
    await reply(ctx.sock, ctx.msg,
      `${icons[signal!] || "⚡"} *${signal}* signal sent to server \`${id}\``
    );
  } catch (e: any) {
    await reply(ctx.sock, ctx.msg, `❌ API error: ${e.message}`);
  }
}
