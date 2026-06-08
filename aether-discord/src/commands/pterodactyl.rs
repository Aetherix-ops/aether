// =============================================================
//  aether-discord — src/commands/pterodactyl.rs
//  Pterodactyl: status, list, start, stop, restart, resources
// =============================================================

use serenity::all::*;

use crate::config::ConfigKey;

pub fn register() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new("ptero")
            .description("Manage Pterodactyl servers")
            .add_option(
                CreateCommandOption::new(CommandOptionType::SubCommand, "list", "List all servers"),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::SubCommand, "status", "Get server status")
                    .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "id", "Server identifier").required(true)),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::SubCommand, "start", "Start a server")
                    .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "id", "Server identifier").required(true)),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::SubCommand, "stop", "Stop a server")
                    .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "id", "Server identifier").required(true)),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::SubCommand, "restart", "Restart a server")
                    .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "id", "Server identifier").required(true)),
            ),
    ]
}

pub async fn handle(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    // Get pterodactyl config
    let data = ctx.data.read().await;
    let cfg_lock = match data.get::<ConfigKey>() {
        Some(c) => c.clone(),
        None => {
            return respond(ctx, interaction, "❌ Bot config not loaded.", true).await;
        }
    };
    let cfg = cfg_lock.read().await;

    let ptero = match &cfg.pterodactyl {
        Some(p) => p.clone(),
        None => {
            return respond(ctx, interaction, "❌ Pterodactyl is not configured. Add `[pterodactyl]` to config.toml.", true).await;
        }
    };

    drop(cfg);
    drop(data);

    // Get subcommand
    let subcommand = match interaction.data.options().iter().find(|o| {
        matches!(o.kind(), CommandOptionType::SubCommand)
    }) {
        Some(s) => s,
        None => return respond(ctx, interaction, "❌ Invalid subcommand.", true).await,
    };

    match subcommand.name.as_str() {
        "list"    => ptero_list(ctx, interaction, &ptero.panel_url, &ptero.api_key).await,
        "status"  => ptero_status(ctx, interaction, &ptero.panel_url, &ptero.api_key, subcommand).await,
        "start"   => ptero_power(ctx, interaction, &ptero.panel_url, &ptero.api_key, subcommand, "start").await,
        "stop"    => ptero_power(ctx, interaction, &ptero.panel_url, &ptero.api_key, subcommand, "stop").await,
        "restart" => ptero_power(ctx, interaction, &ptero.panel_url, &ptero.api_key, subcommand, "restart").await,
        _         => respond(ctx, interaction, "❌ Unknown subcommand.", true).await,
    }
}

async fn respond(ctx: &Context, interaction: &CommandInteraction, msg: &str, ephemeral: bool) -> Result<(), SerenityError> {
    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content(msg).ephemeral(ephemeral)
    )).await
}

async fn api_get(url: &str, api_key: &str, endpoint: &str) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/api/client{}", url, endpoint))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    res.json::<serde_json::Value>().await.map_err(|e| e.to_string())
}

async fn api_post(url: &str, api_key: &str, endpoint: &str, body: serde_json::Value) -> Result<(), String> {
    let client = reqwest::Client::new();
    client
        .post(format!("{}/api/client{}", url, endpoint))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn ptero_list(
    ctx: &Context,
    interaction: &CommandInteraction,
    url: &str,
    key: &str,
) -> Result<(), SerenityError> {
    // Defer to give time for API call
    interaction.defer(&ctx.http).await?;

    match api_get(url, key, "/").await {
        Ok(data) => {
            let servers = data["data"].as_array().cloned().unwrap_or_default();

            if servers.is_empty() {
                return interaction.edit_response(&ctx.http,
                    EditInteractionResponse::new().content("No servers found.")
                ).await.map(|_| ());
            }

            let mut desc = String::new();
            for s in &servers {
                let name = s["attributes"]["name"].as_str().unwrap_or("Unknown");
                let id   = s["attributes"]["identifier"].as_str().unwrap_or("?");
                let node = s["attributes"]["node"].as_str().unwrap_or("?");
                desc.push_str(&format!("`{}` **{}** — Node: {}\n", id, name, node));
            }

            let embed = CreateEmbed::new()
                .title("📋 Pterodactyl Servers")
                .description(desc)
                .color(0x00d2ff)
                .footer(CreateEmbedFooter::new(format!("Total: {} servers", servers.len())));

            interaction.edit_response(&ctx.http,
                EditInteractionResponse::new().embed(embed)
            ).await.map(|_| ())
        }
        Err(e) => interaction.edit_response(&ctx.http,
            EditInteractionResponse::new().content(format!("❌ API error: {}", e))
        ).await.map(|_| ()),
    }
}

async fn ptero_status(
    ctx: &Context,
    interaction: &CommandInteraction,
    url: &str,
    key: &str,
    sub: &ResolvedOption<'_>,
) -> Result<(), SerenityError> {
    let id = sub.value.as_str().unwrap_or_default();

    interaction.defer(&ctx.http).await?;

    match api_get(url, key, &format!("/servers/{}/resources", id)).await {
        Ok(data) => {
            let attr = &data["attributes"];
            let state = attr["current_state"].as_str().unwrap_or("unknown");
            let res   = &attr["resources"];

            let ram  = res["memory_bytes"].as_u64().unwrap_or(0);
            let cpu  = res["cpu_absolute"].as_f64().unwrap_or(0.0);
            let disk = res["disk_bytes"].as_u64().unwrap_or(0);

            let status_icon = match state {
                "running"  => "🟢",
                "starting" => "🟡",
                "stopping" => "🟠",
                _          => "🔴",
            };

            let embed = CreateEmbed::new()
                .title(format!("Server: {}", id))
                .color(if state == "running" { 0x00ff9d } else { 0xff4466 })
                .field("Status", format!("{} {}", status_icon, state), true)
                .field("RAM", format_bytes(ram), true)
                .field("CPU", format!("{:.1}%", cpu), true)
                .field("Disk", format_bytes(disk), true);

            interaction.edit_response(&ctx.http,
                EditInteractionResponse::new().embed(embed)
            ).await.map(|_| ())
        }
        Err(e) => interaction.edit_response(&ctx.http,
            EditInteractionResponse::new().content(format!("❌ API error: {}", e))
        ).await.map(|_| ()),
    }
}

async fn ptero_power(
    ctx: &Context,
    interaction: &CommandInteraction,
    url: &str,
    key: &str,
    sub: &ResolvedOption<'_>,
    signal: &str,
) -> Result<(), SerenityError> {
    let id = sub.value.as_str().unwrap_or_default();

    interaction.defer(&ctx.http).await?;

    let body = serde_json::json!({ "signal": signal });

    match api_post(url, key, &format!("/servers/{}/power", id), body).await {
        Ok(_) => {
            let icon = match signal {
                "start"   => "▶️",
                "stop"    => "⏹️",
                "restart" => "🔄",
                _         => "⚡",
            };
            interaction.edit_response(&ctx.http,
                EditInteractionResponse::new()
                    .content(format!("{} `{}` signal sent to server `{}`", icon, signal, id))
            ).await.map(|_| ())
        }
        Err(e) => interaction.edit_response(&ctx.http,
            EditInteractionResponse::new().content(format!("❌ API error: {}", e))
        ).await.map(|_| ()),
    }
}

fn format_bytes(b: u64) -> String {
    match b {
        b if b >= 1_073_741_824 => format!("{:.1}GB", b as f64 / 1_073_741_824.0),
        b if b >= 1_048_576     => format!("{:.1}MB", b as f64 / 1_048_576.0),
        b if b >= 1_024         => format!("{:.1}KB", b as f64 / 1_024.0),
        _                       => format!("{}B", b),
    }
}
