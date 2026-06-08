// =============================================================
//  aether-discord — src/main.rs
//  A powerful Discord bot built with Rust + Serenity
//  github.com/Aetherix-ops/aether-discord
// =============================================================

mod commands;
mod config;
mod handlers;

use serenity::all::*;
use serenity::framework::standard::StandardFramework;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

pub struct BotData {
    pub config: config::Config,
    pub start_time: chrono::DateTime<chrono::Utc>,
}

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!(
            "✅ Aether is online as {}#{} (ID: {})",
            ready.user.name,
            ready.user.discriminator.unwrap_or_default(),
            ready.user.id
        );

        // Set bot activity
        let activity = ActivityData::playing("Powered by Rust 🦀");
        ctx.set_activity(Some(activity));

        // Register slash commands
        handlers::event::register_commands(&ctx).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        handlers::event::handle_interaction(&ctx, interaction).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        handlers::event::handle_message(&ctx, msg).await;
    }

    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        handlers::event::handle_member_join(&ctx, member).await;
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        _member: Option<Member>,
    ) {
        handlers::event::handle_member_leave(&ctx, guild_id, user).await;
    }
}

#[tokio::main]
async fn main() {
    // Load .env
    dotenv::dotenv().ok();

    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("aether_discord=info".parse().unwrap()),
        )
        .init();

    info!("Starting Aether Discord Bot...");

    // Load config
    let cfg = match config::Config::load() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    let token = cfg.discord.token.clone();
    let prefix = cfg.discord.prefix.clone();

    info!("Config loaded. Prefix: {}", prefix);

    // Build framework
    let framework = StandardFramework::new();

    // Build intents
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES;

    // Build client
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Failed to create Discord client");

    // Store bot data
    {
        let mut data = client.data.write().await;
        data.insert::<config::ConfigKey>(Arc::new(RwLock::new(cfg)));
        data.insert::<commands::StartTimeKey>(
            Arc::new(chrono::Utc::now())
        );
    }

    info!("Connecting to Discord...");

    // Start bot
    if let Err(e) = client.start().await {
        error!("Client error: {:?}", e);
    }
  }
          
