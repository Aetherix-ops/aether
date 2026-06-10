// =============================================================
//  aether-discord — src/handlers/event.rs
//  Global event handler for Discord events
// =============================================================

use serenity::all::*;
use tracing::{error, info};

use crate::commands;
use crate::config::ConfigKey;

/// Register slash commands globally
pub async fn register_commands(ctx: &Context) {
    let commands = commands::build_commands();
    let count = commands.len();

    if let Err(e) = Command::set_global_commands(&ctx.http, commands).await {
        error!("Failed to register commands: {:?}", e);
    } else {
        info!("Registered {} slash commands", count);
    }
}

/// Handle all interactions (slash commands, buttons, etc.)
pub async fn handle_interaction(ctx: &Context, interaction: Interaction) {
    match interaction {
        Interaction::Command(cmd) => {
            commands::handle_command(ctx, &cmd).await;
        }
        Interaction::Component(component) => {
            handle_component(ctx, component).await;
        }
        _ => {}
    }
}

/// Handle button/select menu interactions
/// TODO: Implement actual poll voting logic (store votes, prevent duplicates, etc.)
async fn handle_component(ctx: &Context, interaction: ComponentInteraction) {
    let id = interaction.data.custom_id.as_str();

    if id.starts_with("poll_") {
        // Placeholder response - replace with real voting logic later
        let _ = interaction
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Vote recorded! (voting system coming soon)")
                        .ephemeral(true),
                ),
            )
            .await;
    }
}

/// Handle regular messages (prefix commands / auto-mod)
pub async fn handle_message(ctx: &Context, msg: Message) {
    if msg.author.bot {
        return;
    }

    // Get config
    let data = ctx.data.read().await;
    let cfg_lock = match data.get::<ConfigKey>() {
        Some(c) => c.clone(),
        None => return,
    };
    let cfg = cfg_lock.read().await;

    // Auto-mod
    if cfg.features.as_ref().and_then(|f| f.auto_mod).unwrap_or(false) {
        run_automod(ctx, &msg).await;
    }

    // Prefix commands
    let prefix = &cfg.discord.prefix;
    if let Some(content) = msg.content.strip_prefix(prefix.as_str()) {
        let parts: Vec<&str> = content.splitn(2, ' ').collect();
        let cmd = parts[0].to_lowercase();

        match cmd.as_str() {
            "ping" => {
                if let Err(e) = msg.channel_id.say(&ctx.http, "🏓 Pong!").await {
                    error!("Failed to send ping response: {:?}", e);
                }
            }
            "help" => {
                if let Err(e) = msg.channel_id.say(
                    &ctx.http,
                    "Use `/help` for the full command list!",
                ).await {
                    error!("Failed to send help response: {:?}", e);
                }
            }
            _ => {}
        }
    }
}

/// Simple auto-mod: delete messages with banned words
/// TODO: Move banned words list to config for better flexibility
async fn run_automod(ctx: &Context, msg: &Message) {
    let banned = ["spam", "badword"];
    let lower = msg.content.to_lowercase();

    for word in &banned {
        if lower.contains(word) {
            if let Err(e) = msg.delete(&ctx.http).await {
                error!("Failed to delete message in auto-mod: {:?}", e);
            }
            if let Err(e) = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!(
                        "{} Your message was removed for violating server rules.",
                        msg.author.mention()
                    ),
                )
                .await
            {
                error!("Failed to send auto-mod notification: {:?}", e);
            }
            return;
        }
    }
}

/// Handle member join — send welcome message
pub async fn handle_member_join(ctx: &Context, member: Member) {
    let data = ctx.data.read().await;
    let cfg_lock = match data.get::<ConfigKey>() {
        Some(c) => c.clone(),
        None => return,
    };
    let cfg = cfg_lock.read().await;

    if !cfg.features.as_ref().and_then(|f| f.welcome_enabled).unwrap_or(true) {
        return;
    }

    if let Some(channel_id) = cfg.discord.welcome_channel_id {
        let channel = ChannelId::new(channel_id);
        let msg = cfg
            .discord
            .welcome_message
            .clone()
            .unwrap_or_else(|| "Welcome to the server, %user%!".to_string())
            .replace("%user%", &member.user.mention().to_string())
            .replace("%server%", &member.guild_id.to_string());

        let embed = CreateEmbed::new()
            .title("Welcome!")
            .description(msg)
            .color(0x00d2ff)
            .thumbnail(member.user.avatar_url().unwrap_or_default());

        if let Err(e) = channel
            .send_message(&ctx.http, CreateMessage::new().embed(embed))
            .await
        {
            error!("Failed to send welcome message: {:?}", e);
        }
    }
}

/// Handle member leave
pub async fn handle_member_leave(ctx: &Context, _guild_id: GuildId, user: User) {
    info!("Member left: {}", user.name);
}
