```
 █████╗ ███████╗████████╗██╗  ██╗███████╗██████╗
██╔══██╗██╔════╝╚══██╔══╝██║  ██║██╔════╝██╔══██╗
███████║█████╗     ██║   ███████║█████╗  ██████╔╝
██╔══██║██╔══╝     ██║   ██╔══██║██╔══╝  ██╔══██╗
██║  ██║███████╗   ██║   ██║  ██║███████╗██║  ██║
╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝
```

<div align="center">

![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)
![Rust](https://img.shields.io/badge/discord-Rust%20🦀-orange?style=flat-square&logo=rust)
![TypeScript](https://img.shields.io/badge/whatsapp-TypeScript-3178C6?style=flat-square&logo=typescript)
![Pterodactyl](https://img.shields.io/badge/Pterodactyl-Integration-00d2ff?style=flat-square)

**A powerful dual-platform bot engine.**
Discord bot powered by **Rust 🦀** + WhatsApp bot powered by **TypeScript**.

[Discord Bot](#discord) · [WhatsApp Bot](#whatsapp) · [Installation](#installation)

</div>

---

## Structure

```
aether/
├── discord/      ← Discord bot (Rust + Serenity)
└── whatsapp/     ← WhatsApp bot (TypeScript + Baileys)
```

---

## Discord Bot

**Language:** Rust 🦀 | **Framework:** Serenity

### Features
- General: `/ping` `/info` `/help` `/avatar` `/serverinfo` `/userinfo`
- Moderation: `/ban` `/kick` `/mute` `/unmute` `/warn` `/purge` `/slowmode`
- Utility: `/embed` `/poll` `/remind` `/calc`
- Fun: `/8ball` `/dice` `/roast` `/compliment` `/coinflip` `/rps`
- Pterodactyl: `/ptero list` `/ptero status` `/ptero start` `/ptero stop` `/ptero restart`

### Quick Start

```bash
cd discord
cp config.example.toml config.toml
nano config.toml
cargo build --release
./target/release/aether
```

Full docs: [discord/README.md](discord/README.md)

---

## WhatsApp Bot

**Language:** TypeScript | **Library:** Baileys | **Auth:** Pairing Code

### Features
- General: `!help` `!ping` `!info` `!uptime` `!owner`
- Moderation: `!kick` `!add` `!promote` `!demote` `!everyone` `!antilink` `!mute` `!unmute`
- Utility: `!sticker` `!toimg` `!qr` `!calc` `!weather`
- Downloader: `!yt` `!ytv` `!tiktok` `!ig`
- AI: `!ai` `!chat` _(requires OpenRouter key)_
- Pterodactyl: `!ptero list/status/start/stop/restart`

### Quick Start

```bash
cd whatsapp
npm install
cp config.example.json config.json
nano config.json
npm run dev
```

Then enter the **pairing code** shown in terminal into WhatsApp → Linked Devices → Link with phone number.

Full docs: [whatsapp/README.md](whatsapp/README.md)

---

## Pterodactyl Integration

Both bots support Pterodactyl Panel integration out of the box.

**Discord:** `/ptero list`, `/ptero status <id>`, `/ptero start/stop/restart <id>`

**WhatsApp:** `!ptero list`, `!ptero status <id>`, `!ptero start/stop/restart <id>`

---

## File Structure

```
aether/
├── discord/
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── commands/
│   │   │   ├── general.rs
│   │   │   ├── moderation.rs
│   │   │   ├── utility.rs
│   │   │   ├── fun.rs
│   │   │   └── pterodactyl.rs
│   │   └── handlers/
│   │       └── event.rs
│   ├── Cargo.toml
│   └── config.example.toml
│
└── whatsapp/
    ├── src/
    │   ├── index.ts
    │   ├── config.ts
    │   ├── commands/
    │   │   ├── general.ts
    │   │   ├── moderation.ts
    │   │   ├── utility.ts
    │   │   ├── downloader.ts
    │   │   ├── ai.ts
    │   │   └── pterodactyl.ts
    │   ├── handlers/
    │   │   └── message.ts
    │   └── utils/
    │       ├── logger.ts
    │       └── helpers.ts
    ├── package.json
    ├── tsconfig.json
    └── config.example.json
```

---

## Related Repositories

| Repository | Description |
|---|---|
| [ptero-api](https://github.com/Aetherix-ops/ptero-api) | Pterodactyl API library |
| [ptero-scripts](https://github.com/Aetherix-ops/ptero-scripts) | Pterodactyl shell scripts |
| [ptero-eggs](https://github.com/Aetherix-ops/ptero-eggs) | Pterodactyl egg collection |
| [novastar-theme](https://github.com/Aetherix-ops/novastar-theme) | Pterodactyl panel theme |

---

## License

MIT — by [Aetherix-ops](https://github.com/Aetherix-ops)
