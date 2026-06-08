// =============================================================
//  aether-discord — src/commands/mod.rs
//  Command registry and shared types
// =============================================================

pub mod fun;
pub mod general;
pub mod moderation;
pub mod pterodactyl;
pub mod utility;

use serenity::all::*;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

pub struct StartTimeKey;
impl TypeMapKey for StartTimeKey {
    type Value = Arc<chrono::DateTime<chrono::Utc>>;
}

/// Build all slash commands for registration
pub fn build_commands() -> Vec<CreateCommand> {
    let mut commands = vec![];

    commands.extend(general::register());
    commands.extend(moderation::register());
    commands.extend(utility::register());
    commands.extend(fun::register());
    commands.extend(pterodactyl::register());

    commands
}

/// Route slash command to the correct handler
pub async fn handle_command(ctx: &Context, interaction: &CommandInteraction) {
    let name = interaction.data.name.as_str();

    let result = match name {
        // General
        "ping" | "info" | "help" | "avatar" | "serverinfo" | "userinfo" => {
            general::handle(ctx, interaction).await
        }

        // Moderation
        "ban" | "kick" | "mute" | "unmute" | "warn" | "purge" | "slowmode" => {
            moderation::handle(ctx, interaction).await
        }

        // Utility
        "embed" | "poll" | "remind" | "calc" => {
            utility::handle(ctx, interaction).await
        }

        // Fun
        "8ball" | "dice" | "roast" | "compliment" | "coinflip" | "rps" => {
            fun::handle(ctx, interaction).await
        }

        // Pterodactyl
        "ptero" => pterodactyl::handle(ctx, interaction).await,

        _ => {
            let _ = interaction
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("Unknown command.")
                            .ephemeral(true),
                    ),
                )
                .await;
            Ok(())
        }
    };

    if let Err(e) = result {
        tracing::error!("Command error [{}]: {:?}", name, e);
    }
}
