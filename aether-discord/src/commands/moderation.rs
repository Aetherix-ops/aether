// =============================================================
//  aether-discord — src/commands/moderation.rs
//  Moderation: ban, kick, mute, unmute, warn, purge, slowmode
// =============================================================

use serenity::all::*;

pub fn register() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new("ban")
            .description("Ban a member")
            .default_member_permissions(Permissions::BAN_MEMBERS)
            .add_option(CreateCommandOption::new(CommandOptionType::User, "user", "User to ban").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "reason", "Reason").required(false)),

        CreateCommand::new("kick")
            .description("Kick a member")
            .default_member_permissions(Permissions::KICK_MEMBERS)
            .add_option(CreateCommandOption::new(CommandOptionType::User, "user", "User to kick").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "reason", "Reason").required(false)),

        CreateCommand::new("mute")
            .description("Timeout a member")
            .default_member_permissions(Permissions::MODERATE_MEMBERS)
            .add_option(CreateCommandOption::new(CommandOptionType::User, "user", "User to mute").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::Integer, "duration", "Duration in minutes").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "reason", "Reason").required(false)),

        CreateCommand::new("unmute")
            .description("Remove timeout from a member")
            .default_member_permissions(Permissions::MODERATE_MEMBERS)
            .add_option(CreateCommandOption::new(CommandOptionType::User, "user", "User to unmute").required(true)),

        CreateCommand::new("warn")
            .description("Warn a member")
            .default_member_permissions(Permissions::MODERATE_MEMBERS)
            .add_option(CreateCommandOption::new(CommandOptionType::User, "user", "User to warn").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "reason", "Reason").required(true)),

        CreateCommand::new("purge")
            .description("Delete multiple messages")
            .default_member_permissions(Permissions::MANAGE_MESSAGES)
            .add_option(CreateCommandOption::new(CommandOptionType::Integer, "amount", "Number of messages (1-100)").required(true)),

        CreateCommand::new("slowmode")
            .description("Set channel slowmode")
            .default_member_permissions(Permissions::MANAGE_CHANNELS)
            .add_option(CreateCommandOption::new(CommandOptionType::Integer, "seconds", "Slowmode in seconds (0 to disable)").required(true)),
    ]
}

pub async fn handle(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    match interaction.data.name.as_str() {
        "ban"      => ban(ctx, interaction).await,
        "kick"     => kick(ctx, interaction).await,
        "mute"     => mute(ctx, interaction).await,
        "unmute"   => unmute(ctx, interaction).await,
        "warn"     => warn(ctx, interaction).await,
        "purge"    => purge(ctx, interaction).await,
        "slowmode" => slowmode(ctx, interaction).await,
        _          => Ok(()),
    }
}

fn get_option_user(interaction: &CommandInteraction, name: &str) -> Option<UserId> {
    interaction.data.options().iter()
        .find(|o| o.name == name)
        .and_then(|o| o.value.as_user_id())
}

fn get_option_str<'a>(interaction: &'a CommandInteraction, name: &str) -> Option<&'a str> {
    interaction.data.options().iter()
        .find(|o| o.name == name)
        .and_then(|o| o.value.as_str())
}

fn get_option_int(interaction: &CommandInteraction, name: &str) -> Option<i64> {
    interaction.data.options().iter()
        .find(|o| o.name == name)
        .and_then(|o| o.value.as_i64())
}

async fn respond(ctx: &Context, interaction: &CommandInteraction, msg: &str, ephemeral: bool) -> Result<(), SerenityError> {
    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content(msg).ephemeral(ephemeral)
    )).await
}

async fn ban(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let guild_id = match interaction.guild_id {
        Some(id) => id,
        None => return respond(ctx, interaction, "This command can only be used in a server.", true).await,
    };

    let user_id = match get_option_user(interaction, "user") {
        Some(id) => id,
        None => return respond(ctx, interaction, "Please specify a user.", true).await,
    };

    let reason = get_option_str(interaction, "reason").unwrap_or("No reason provided");

    match guild_id.ban_with_reason(&ctx.http, user_id, 0, reason).await {
        Ok(_) => respond(ctx, interaction, &format!("✅ Banned <@{}> — Reason: {}", user_id, reason), false).await,
        Err(e) => respond(ctx, interaction, &format!("❌ Failed to ban: {}", e), true).await,
    }
}

async fn kick(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let guild_id = match interaction.guild_id {
        Some(id) => id,
        None => return respond(ctx, interaction, "This command can only be used in a server.", true).await,
    };

    let user_id = match get_option_user(interaction, "user") {
        Some(id) => id,
        None => return respond(ctx, interaction, "Please specify a user.", true).await,
    };

    let reason = get_option_str(interaction, "reason").unwrap_or("No reason provided");

    match guild_id.kick_with_reason(&ctx.http, user_id, reason).await {
        Ok(_) => respond(ctx, interaction, &format!("✅ Kicked <@{}> — Reason: {}", user_id, reason), false).await,
        Err(e) => respond(ctx, interaction, &format!("❌ Failed to kick: {}", e), true).await,
    }
}

async fn mute(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let guild_id = match interaction.guild_id {
        Some(id) => id,
        None => return respond(ctx, interaction, "Server only.", true).await,
    };

    let user_id = match get_option_user(interaction, "user") {
        Some(id) => id,
        None => return respond(ctx, interaction, "Please specify a user.", true).await,
    };

    let duration = get_option_int(interaction, "duration").unwrap_or(10);
    let reason = get_option_str(interaction, "reason").unwrap_or("No reason provided");
    let until = chrono::Utc::now() + chrono::Duration::minutes(duration);

    let edit = EditMember::new().disable_communication_until(until.into());

    match guild_id.edit_member(&ctx.http, user_id, edit).await {
        Ok(_) => respond(ctx, interaction, &format!(
            "🔇 Muted <@{}> for {} minutes — Reason: {}", user_id, duration, reason
        ), false).await,
        Err(e) => respond(ctx, interaction, &format!("❌ Failed to mute: {}", e), true).await,
    }
}

async fn unmute(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let guild_id = match interaction.guild_id {
        Some(id) => id,
        None => return respond(ctx, interaction, "Server only.", true).await,
    };

    let user_id = match get_option_user(interaction, "user") {
        Some(id) => id,
        None => return respond(ctx, interaction, "Please specify a user.", true).await,
    };

    let edit = EditMember::new().enable_communication();

    match guild_id.edit_member(&ctx.http, user_id, edit).await {
        Ok(_) => respond(ctx, interaction, &format!("🔊 Unmuted <@{}>", user_id), false).await,
        Err(e) => respond(ctx, interaction, &format!("❌ Failed to unmute: {}", e), true).await,
    }
}

async fn warn(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let user_id = match get_option_user(interaction, "user") {
        Some(id) => id,
        None => return respond(ctx, interaction, "Please specify a user.", true).await,
    };

    let reason = get_option_str(interaction, "reason").unwrap_or("No reason provided");

    respond(ctx, interaction, &format!(
        "⚠️ <@{}> has been warned — Reason: {}", user_id, reason
    ), false).await
}

async fn purge(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let amount = get_option_int(interaction, "amount").unwrap_or(10).min(100).max(1) as u8;

    let messages = interaction.channel_id
        .messages(&ctx.http, GetMessages::new().limit(amount))
        .await?;

    let ids: Vec<MessageId> = messages.iter().map(|m| m.id).collect();
    let count = ids.len();

    interaction.channel_id.delete_messages(&ctx.http, ids).await?;

    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(format!("🗑️ Deleted {} messages.", count))
            .ephemeral(true)
    )).await
}

async fn slowmode(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let seconds = get_option_int(interaction, "seconds").unwrap_or(0) as u16;

    let edit = EditChannel::new().rate_limit_per_user(seconds);
    interaction.channel_id.edit(&ctx.http, edit).await?;

    let msg = if seconds == 0 {
        "✅ Slowmode disabled.".to_string()
    } else {
        format!("✅ Slowmode set to {} seconds.", seconds)
    };

    respond(ctx, interaction, &msg, false).await
      }
                                
