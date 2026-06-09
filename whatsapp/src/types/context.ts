// =============================================================
//  aether-wa — src/types/context.ts
//  Command context type
// =============================================================

import { WASocket, proto } from "@whiskeysockets/baileys";
import { Config } from "../config";

export interface CommandContext {
  sock: WASocket;
  msg: proto.IWebMessageInfo;
  args: string[];
  chatId: string;
  sender: string;
  group: boolean;
  config: Config;
  startTime: number;
}
