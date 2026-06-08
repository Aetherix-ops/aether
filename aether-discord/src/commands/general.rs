// =============================================================
//  aether-discord — src/commands/general.rs
//  General commands: ping, help, info, avatar, serverinfo
// =============================================================

use serenity::all::*;
use tracing::error;

use crate::commands::StartTimeKey;
use crate::config::ConfigKey;

pub fn register() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new("ping")
            .description("Check bot latency"),

        CreateCommand::new("info")
            .description("Show bot information"),

        CreateCommand::new("help")
            .description("Show all available commands"),

        CreateCommand::new("avatar")
            .description("Get a user's avatar")
            .add_option(
                CreateCommandOption::new(CommandOptionType::User, "user", "Target user")
                    .required(false),
            ),

        CreateCommand::new("serverinfo")
            .description("Show server information"),

        CreateCommand::new("userinfo")
            .description("Show user information")
            .add_option(
                CreateCommandOption::new(CommandOptionType::User, "user", "Target user")
                    .required(false),
            ),
    ]
}

pub async fn handle(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    match interaction.data.name.as_str() {
        "ping"       => ping(ctx, interaction).await,
        "info"       => info_cmd(ctx, interaction).await,
        "help"       => help(ctx, interaction).await,
        "avatar"     => avatar(ctx, interaction).await,
        "serverinfo" => serverinfo(ctx, interaction).await,
        "userinfo"   => userinfo(ctx, interaction).await,
        _            => Ok(()),
    }
}

async fn ping(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let start = std::time::Instant::now();

    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("🏓 Pinging..."),
            ),
        )
        .await?;

    let latency = start.elapsed().as_millis();

    interaction
        .edit_response(
            &ctx.http,
            EditInteractionResponse::new().content(format!(
                "🏓 **Pong!**\n> Latency: `{}ms`",
                latency
            )),
        )
        .await?;

    Ok(())
}

async fn info_cmd(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let data = ctx.data.read().await;
    let uptime = if let Some(start) = data.get::<StartTimeKey>() {
        let elapsed = chrono::Utc::now().signed_duration_since(**start);
        format!(
            "{}d {}h {}m",
            elapsed.num_days(),
            elapsed.num_hours() % 24,
            elapsed.num_minutes() % 60
        )
    } else {
        "Unknown".to_string()
    };

    let embed = CreateEmbed::new()
        .title("⚡ Aether Discord Bot")
        .description("A powerful Discord bot built with **Rust** 🦀")
        .color(0x00d2ff)
        .field("Version", "1.0.0", true)
        .field("Language", "Rust 🦀", true)
        .field("Framework", "Serenity", true)
        .field("Uptime", uptime, true)
        .field("Author", "Aetherix-ops", true)
        .field("Source", "[GitHub](https://github.com/Aetherix-ops/aether-discord)", true)
        .footer(CreateEmbedFooter::new("github.com/Aetherix-ops/aether-discord"));

    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed),
            ),
        )
        .await
}

async fn help(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let embed = CreateEmbed::new()
        .title("📖 Aether — Command List")
        .color(0x00d2ff)
        .field(
            "General",
            "`/ping` `/info` `/help` `/avatar` `/serverinfo` `/userinfo`",
            false,
        )
        .field(
            "Moderation",
            "`/ban` `/kick` `/mute` `/unmute` `/warn` `/purge` `/slowmode`",
            false,
        )
        .field(
            "Utility",
            "`/embed` `/poll` `/remind` `/calc`",
            false,
        )
        .field(
            "Fun",
            "`/8ball` `/dice` `/roast` `/compliment` `/coinflip` `/rps`",
            false,
        )
        .field(
            "Pterodactyl",
            "`/ptero status` `/ptero list` `/ptero start` `/ptero stop` `/ptero restart`",
            false,
        )
        .footer(CreateEmbedFooter::new("Aether v1.0.0 • Built with Rust 🦀"));

    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(embed)
                    .ephemeral(true),
            ),
        )
        .await
}

async fn avatar(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let user = interaction
        .data
        .options()
        .iter()
        .find(|o| o.name == "user")
        .and_then(|o| o.value.as_user_id())
        .map(|id| id.to_user_cached(&ctx.cache).map(|u| u.clone()))
        .flatten()
        .unwrap_or_else(|| interaction.user.clone());

    let avatar_url = user
        .avatar_url()
        .unwrap_or_else(|| user.default_avatar_url());

    let embed = CreateEmbed::new()
        .title(format!("{}'s Avatar", user.name))
        .image(&avatar_url)
        .color(0x00d2ff)
        .url(&avatar_url);

    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed),
            ),
        )
        .await
}

async fn serverinfo(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let guild_id = match interaction.guild_id {
        Some(id) => id,
        None => {
            return interaction
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("This command can only be used in a server.")
                            .ephemeral(true),
                    ),
                )
                .await;
        }
    };

    let guild = match guild_id.to_guild_cached(&ctx.cache) {
        Some(g) => g.clone(),
        None => {
            return interaction
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("Could not fetch server info.")
                            .ephemeral(true),
                    ),
                )
                .await;
        }
    };

    let icon = guild
        .icon_url()
        .unwrap_or_else(|| "https://cdn.discordapp.com/embed/avatars/0.png".to_string());

    let embed = CreateEmbed::new()
        .title(&guild.name)
        .thumbnail(&icon)
        .color(0x00d2ff)
        .field("Owner", format!("<@{}>", guild.owner_id), true)
        .field("Members", guild.member_count.to_string(), true)
        .field("Channels", guild.channels.len().to_string(), true)
        .field("Roles", guild.roles.len().to_string(), true)
        .field("Boosts", guild.premium_subscription_count.unwrap_or(0).to_string(), true)
        .field("Server ID", guild.id.to_string(), true)
        .footer(CreateEmbedFooter::new(format!(
            "Created: {}",
            guild.id.created_at().format("%Y-%m-%d")
        )));

    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed),
            ),
        )
        .await
}

async fn userinfo(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let user = interaction
        .data
        .options()
        .iter()
        .find(|o| o.name == "user")
        .and_then(|o| o.value.as_user_id())
        .map(|id| id.to_user_cached(&ctx.cache).map(|u| u.clone()))
        .flatten()
        .unwrap_or_else(|| interaction.user.clone());

    let avatar = user.avatar_url().unwrap_or_else(|| user.default_avatar_url());

    let embed = CreateEmbed::new()
        .title(format!("{}", user.name))
        .thumbnail(&avatar)
        .color(0x00d2ff)
        .field("User ID", user.id.to_string(), true)
        .field("Bot", if user.bot { "Yes" } else { "No" }, true)
        .field(
            "Account Created",
            user.id.created_at().format("%Y-%m-%d").to_string(),
            true,
        );

    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed),
            ),
        )
        .await
      }
      
