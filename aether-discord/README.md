```
 █████╗ ███████╗████████╗██╗  ██╗███████╗██████╗
██╔══██╗██╔════╝╚══██╔══╝██║  ██║██╔════╝██╔══██╗
███████║█████╗     ██║   ███████║█████╗  ██████╔╝
██╔══██║██╔══╝     ██║   ██╔══██║██╔══╝  ██╔══██╗
██║  ██║███████╗   ██║   ██║  ██║███████╗██║  ██║
╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝
         D I S C O R D   B O T
```

<div align="center">

![Version](https://img.shields.io/badge/version-1.0.0-00d2ff?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange?style=flat-square&logo=rust)
![Serenity](https://img.shields.io/badge/Serenity-0.12-00d2ff?style=flat-square)
![CI](https://img.shields.io/github/actions/workflow/status/Aetherix-ops/aether-discord/ci.yml?style=flat-square&label=build)

**A powerful Discord bot built with Rust 🦀**
Fast, reliable, and feature-rich. Powered by Serenity framework.

</div>

---

## Features

### General
- `/ping` — Check bot latency
- `/info` — Bot information and uptime
- `/help` — Full command list
- `/avatar` — Get user avatar
- `/serverinfo` — Server details
- `/userinfo` — User details

### Moderation
- `/ban` — Ban a member
- `/kick` — Kick a member
- `/mute` — Timeout a member
- `/unmute` — Remove timeout
- `/warn` — Warn a member
- `/purge` — Bulk delete messages
- `/slowmode` — Set channel slowmode

### Utility
- `/embed` — Create custom embed
- `/poll` — Create a poll
- `/remind` — Set a reminder
- `/calc` — Calculate math expression

### Fun
- `/8ball` — Magic 8-ball
- `/dice` — Roll a dice
- `/roast` — Roast a user
- `/compliment` — Compliment a user
- `/coinflip` — Flip a coin
- `/rps` — Rock Paper Scissors

### Pterodactyl Integration
- `/ptero list` — List all servers
- `/ptero status <id>` — Server status + resources
- `/ptero start <id>` — Start a server
- `/ptero stop <id>` — Stop a server
- `/ptero restart <id>` — Restart a server

---

## Requirements

- Rust 1.75+
- Discord Bot Token
- Pterodactyl Panel (optional)

---

## Installation

    git clone https://github.com/Aetherix-ops/aether-discord.git
    cd aether-discord

    cp config.example.toml config.toml
    nano config.toml

    cargo build --release
    ./target/release/aether

---

## Configuration

Edit `config.toml`:

```toml
[discord]
token = "your_discord_bot_token"
prefix = "!"
owner_id = 123456789012345678
log_channel_id = 123456789012345678
welcome_channel_id = 123456789012345678
welcome_message = "Welcome to %server%, %user%!"

[pterodactyl]
panel_url = "https://panel.yourdomain.com"
api_key = "your_client_api_key"

[features]
auto_mod = false
welcome_enabled = true
economy_enabled = false
```

Or use environment variables:

```bash
export DISCORD_TOKEN=your_token
export DISCORD_PREFIX=!
export PTERO_URL=https://panel.yourdomain.com
export PTERO_API_KEY=your_api_key
```

---

## Getting a Bot Token

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Create New Application
3. Go to Bot tab
4. Reset Token and copy it
5. Enable: Server Members Intent, Message Content Intent

---

## Invite Bot to Server

Replace `YOUR_CLIENT_ID` with your bot's Application ID:

```
https://discord.com/api/oauth2/authorize?client_id=YOUR_CLIENT_ID&permissions=8&scope=bot%20applications.commands
```

---

## File Structure

    aether-discord/
    |- src/
    |   |- main.rs
    |   |- config.rs
    |   |- commands/
    |   |   |- mod.rs
    |   |   |- general.rs
    |   |   |- moderation.rs
    |   |   |- utility.rs
    |   |   |- fun.rs
    |   |   |- pterodactyl.rs
    |   |- handlers/
    |       |- mod.rs
    |       |- event.rs
    |- Cargo.toml
    |- config.example.toml
    |- README.md

---

## Related

- [aether-wa](https://github.com/Aetherix-ops/aether-wa) — WhatsApp bot companion
- [ptero-api](https://github.com/Aetherix-ops/ptero-api) — Pterodactyl API library
- [ptero-scripts](https://github.com/Aetherix-ops/ptero-scripts) — Pterodactyl shell scripts

---

## License

MIT — by [Aetherix-ops](https://github.com/Aetherix-ops)
