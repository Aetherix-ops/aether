// =============================================================
//  aether-discord — src/commands/utility.rs
//  Utility: embed, poll, remind, calc
// =============================================================

use serenity::all::*;

pub fn register() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new("embed")
            .description("Create a custom embed message")
            .default_member_permissions(Permissions::MANAGE_MESSAGES)
            .add_option(CreateCommandOption::new(CommandOptionType::String, "title", "Embed title").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "description", "Embed description").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "color", "Hex color (e.g. 00d2ff)").required(false)),

        CreateCommand::new("poll")
            .description("Create a poll")
            .add_option(CreateCommandOption::new(CommandOptionType::String, "question", "Poll question").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "options", "Options separated by | (e.g. Yes|No|Maybe)").required(true)),

        CreateCommand::new("remind")
            .description("Set a reminder")
            .add_option(CreateCommandOption::new(CommandOptionType::Integer, "minutes", "Remind after X minutes").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "message", "Reminder message").required(true)),

        CreateCommand::new("calc")
            .description("Calculate a math expression")
            .add_option(CreateCommandOption::new(CommandOptionType::String, "expression", "Math expression (e.g. 2 + 2 * 3)").required(true)),
    ]
}

pub async fn handle(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    match interaction.data.name.as_str() {
        "embed"  => embed(ctx, interaction).await,
        "poll"   => poll(ctx, interaction).await,
        "remind" => remind(ctx, interaction).await,
        "calc"   => calc(ctx, interaction).await,
        _        => Ok(()),
    }
}

async fn embed(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let title = interaction.data.options().iter()
        .find(|o| o.name == "title")
        .and_then(|o| o.value.as_str())
        .unwrap_or("Embed");

    let description = interaction.data.options().iter()
        .find(|o| o.name == "description")
        .and_then(|o| o.value.as_str())
        .unwrap_or("");

    let color_str = interaction.data.options().iter()
        .find(|o| o.name == "color")
        .and_then(|o| o.value.as_str())
        .unwrap_or("00d2ff");

    let color = u32::from_str_radix(color_str.trim_start_matches('#'), 16).unwrap_or(0x00d2ff);

    let embed = CreateEmbed::new()
        .title(title)
        .description(description)
        .color(color)
        .footer(CreateEmbedFooter::new(format!("Requested by {}", interaction.user.name)));

    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(embed)
    )).await
}

async fn poll(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let question = interaction.data.options().iter()
        .find(|o| o.name == "question")
        .and_then(|o| o.value.as_str())
        .unwrap_or("Poll");

    let options_str = interaction.data.options().iter()
        .find(|o| o.name == "options")
        .and_then(|o| o.value.as_str())
        .unwrap_or("Yes|No");

    let emojis = ["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣", "9️⃣", "🔟"];
    let options: Vec<&str> = options_str.split('|').take(10).collect();

    let description = options.iter().enumerate()
        .map(|(i, opt)| format!("{} {}", emojis[i], opt.trim()))
        .collect::<Vec<_>>()
        .join("\n");

    let embed = CreateEmbed::new()
        .title(format!("📊 {}", question))
        .description(description)
        .color(0x00d2ff)
        .footer(CreateEmbedFooter::new(format!("Poll by {}", interaction.user.name)));

    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(embed)
    )).await
}

async fn remind(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let minutes = interaction.data.options().iter()
        .find(|o| o.name == "minutes")
        .and_then(|o| o.value.as_i64())
        .unwrap_or(5);

    let message = interaction.data.options().iter()
        .find(|o| o.name == "message")
        .and_then(|o| o.value.as_str())
        .unwrap_or("Reminder!")
        .to_string();

    let user_id = interaction.user.id;
    let channel_id = interaction.channel_id;
    let http = ctx.http.clone();

    // Acknowledge first
    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(format!("⏰ I'll remind you in {} minute(s)!", minutes))
            .ephemeral(true)
    )).await?;

    // Spawn reminder task
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(minutes as u64 * 60)).await;
        let _ = channel_id.say(&http, format!(
            "⏰ <@{}> Reminder: **{}**", user_id, message
        )).await;
    });

    Ok(())
}

async fn calc(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    let expr = interaction.data.options().iter()
        .find(|o| o.name == "expression")
        .and_then(|o| o.value.as_str())
        .unwrap_or("0");

    // Simple safe evaluator (only basic math)
    let result = eval_expr(expr);

    let msg = match result {
        Ok(val) => format!("🧮 `{}` = **{}**", expr, val),
        Err(e)  => format!("❌ Invalid expression: {}", e),
    };

    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content(msg)
    )).await
}

fn eval_expr(expr: &str) -> Result<f64, String> {
    // Sanitize: only allow numbers, operators, spaces, dots, parentheses
    if !expr.chars().all(|c| c.is_ascii_digit() || "+-*/%. ()".contains(c)) {
        return Err("Only basic math operators allowed".to_string());
    }

    // Very basic evaluator (no external crates)
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    if tokens.len() == 3 {
        let a: f64 = tokens[0].parse().map_err(|_| "Invalid number")?;
        let b: f64 = tokens[2].parse().map_err(|_| "Invalid number")?;
        return match tokens[1] {
            "+" => Ok(a + b),
            "-" => Ok(a - b),
            "*" => Ok(a * b),
            "/" => if b == 0.0 { Err("Division by zero".to_string()) } else { Ok(a / b) },
            "%" => Ok(a % b),
            _   => Err("Unknown operator".to_string()),
        };
    }

    Err("Format: number operator number (e.g. 10 + 5)".to_string())
}
