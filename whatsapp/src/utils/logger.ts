// =============================================================
//  aether-wa — src/utils/logger.ts
// =============================================================

const C = {
  reset:  "\x1b[0m",
  red:    "\x1b[31m",
  green:  "\x1b[32m",
  yellow: "\x1b[33m",
  cyan:   "\x1b[36m",
  white:  "\x1b[1;37m",
  dim:    "\x1b[2m",
};

function timestamp(): string {
  return new Date().toISOString().replace("T", " ").slice(0, 19);
}

export const logger = {
  info:  (msg: string, ...args: unknown[]) =>
    console.log(`${C.dim}[${timestamp()}]${C.reset} ${C.cyan}[INFO]${C.reset} ${msg}`, ...args),
  warn:  (msg: string, ...args: unknown[]) =>
    console.log(`${C.dim}[${timestamp()}]${C.reset} ${C.yellow}[WARN]${C.reset} ${msg}`, ...args),
  error: (msg: string, ...args: unknown[]) =>
    console.error(`${C.dim}[${timestamp()}]${C.reset} ${C.red}[ERR]${C.reset} ${msg}`, ...args),
  ok:    (msg: string, ...args: unknown[]) =>
    console.log(`${C.dim}[${timestamp()}]${C.reset} ${C.green}[OK]${C.reset} ${msg}`, ...args),
  cmd:   (msg: string, ...args: unknown[]) =>
    console.log(`${C.dim}[${timestamp()}]${C.reset} ${C.white}[CMD]${C.reset} ${msg}`, ...args),
};
  
