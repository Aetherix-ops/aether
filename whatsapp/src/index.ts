// =============================================================
//  aether-wa — src/index.ts
//  A powerful WhatsApp bot built with TypeScript + Baileys
//  github.com/Aetherix-ops/aether
// =============================================================

import makeWASocket, {
  useMultiFileAuthState,
  DisconnectReason,
  fetchLatestBaileysVersion,
  makeCacheableSignalKeyStore,
  WASocket,
  BaileysEventMap,
} from "@whiskeysockets/baileys";
import { Boom } from "@hapi/boom";
import pino from "pino";
import path from "path";
import fs from "fs";

import { loadConfig } from "./config";
import { handleMessage } from "./handlers/message";
import { logger } from "./utils/logger";

const AUTH_DIR = path.join(process.cwd(), "session");

async function startBot(): Promise<void> {
  const config = loadConfig();

  logger.info("Starting Aether WhatsApp Bot...");
  logger.info(`Prefix: ${config.prefix}`);

  if (!fs.existsSync(AUTH_DIR)) {
    fs.mkdirSync(AUTH_DIR, { recursive: true });
  }

  const { state, saveCreds } = await useMultiFileAuthState(AUTH_DIR);
  const { version } = await fetchLatestBaileysVersion();
  logger.info(`Baileys version: ${version.join(".")}`);

  const sock: WASocket = makeWASocket({
    version,
    auth: {
      creds: state.creds,
      keys: makeCacheableSignalKeyStore(state.keys, pino({ level: "silent" })),
    },
    printQRInTerminal: false,
    logger: pino({ level: "silent" }),
    browser: ["Aether Bot", "Chrome", "1.0.0"],
    syncFullHistory: false,
    markOnlineOnConnect: false,
  });

  if (!sock.authState.creds.registered) {
    const phoneNumber = config.phoneNumber;

    if (!phoneNumber) {
      logger.error("PHONE_NUMBER not set in config!");
      process.exit(1);
    }

    await new Promise((resolve) => setTimeout(resolve, 3000));

    try {
      const code = await sock.requestPairingCode(
        phoneNumber.replace(/[^0-9]/g, "")
      );
      const formatted = code.match(/.{1,4}/g)?.join("-") || code;

      console.log("\n");
      console.log("╔══════════════════════════════════════╗");
      console.log("║       AETHER — PAIRING CODE          ║");
      console.log("╠══════════════════════════════════════╣");
      console.log(`║                                      ║`);
      console.log(`║   Code: ${formatted.padEnd(28)}║`);
      console.log(`║                                      ║`);
      console.log("╠══════════════════════════════════════╣");
      console.log("║  1. Open WhatsApp on your phone      ║");
      console.log("║  2. Go to Linked Devices              ║");
      console.log("║  3. Tap Link with phone number        ║");
      console.log("║  4. Enter the code above              ║");
      console.log("╚══════════════════════════════════════╝");
      console.log("\n");
    } catch (e) {
      logger.error("Failed to get pairing code:", e);
    }
  }

  sock.ev.on("connection.update", async (update) => {
    const { connection, lastDisconnect } = update;

    if (connection === "close") {
      const reason = (lastDisconnect?.error as Boom)?.output?.statusCode;
      const shouldReconnect = reason !== DisconnectReason.loggedOut;

      logger.warn(`Connection closed. Reason: ${reason}`);

      if (shouldReconnect) {
        logger.info("Reconnecting...");
        setTimeout(() => startBot(), 5000);
      } else {
        logger.error("Logged out. Delete session folder and restart.");
        fs.rmSync(AUTH_DIR, { recursive: true, force: true });
        process.exit(1);
      }
    }

    if (connection === "open") {
      logger.info(`✅ Connected as ${sock.user?.name || sock.user?.id}`);
      logger.info(`Phone: ${sock.user?.id?.split(":")[0]}`);
    }

    if (connection === "connecting") {
      logger.info("Connecting to WhatsApp...");
    }
  });

  sock.ev.on("creds.update", saveCreds);

  sock.ev.on("messages.upsert", async ({ messages, type }) => {
    if (type !== "notify") return;

    for (const msg of messages) {
      if (!msg.message) continue;
      if (msg.key.fromMe) continue;
      await handleMessage(sock, msg);
    }
  });

  process.on("SIGINT", async () => {
    logger.info("Shutting down Aether...");
    await sock.logout();
    process.exit(0);
  });
}

startBot().catch((e) => {
  logger.error("Fatal error:", e);
  process.exit(1);
});
