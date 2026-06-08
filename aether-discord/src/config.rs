// =============================================================
//  aether-discord — src/config.rs
//  Configuration loader from config.toml or environment
// =============================================================

use serde::{Deserialize, Serialize};
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ConfigKey;
impl TypeMapKey for ConfigKey {
    type Value = Arc<RwLock<Config>>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub discord: DiscordConfig,
    pub pterodactyl: Option<PterodactylConfig>,
    pub features: Option<FeaturesConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiscordConfig {
    pub token: String,
    pub prefix: String,
    pub owner_id: Option<u64>,
    pub log_channel_id: Option<u64>,
    pub welcome_channel_id: Option<u64>,
    pub welcome_message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PterodactylConfig {
    pub panel_url: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FeaturesConfig {
    pub auto_mod: Option<bool>,
    pub welcome_enabled: Option<bool>,
    pub economy_enabled: Option<bool>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Try config.toml first
        if std::path::Path::new("config.toml").exists() {
            let content = std::fs::read_to_string("config.toml")?;
            let config: Config = toml::from_str(&content)?;
            return Ok(config);
        }

        // Fall back to environment variables
        let token = std::env::var("DISCORD_TOKEN")
            .map_err(|_| "DISCORD_TOKEN not set. Create config.toml or set env vars.")?;

        Ok(Config {
            discord: DiscordConfig {
                token,
                prefix: std::env::var("DISCORD_PREFIX").unwrap_or_else(|_| "!".to_string()),
                owner_id: std::env::var("OWNER_ID")
                    .ok()
                    .and_then(|v| v.parse().ok()),
                log_channel_id: std::env::var("LOG_CHANNEL_ID")
                    .ok()
                    .and_then(|v| v.parse().ok()),
                welcome_channel_id: std::env::var("WELCOME_CHANNEL_ID")
                    .ok()
                    .and_then(|v| v.parse().ok()),
                welcome_message: std::env::var("WELCOME_MESSAGE").ok(),
            },
            pterodactyl: match (
                std::env::var("PTERO_URL"),
                std::env::var("PTERO_API_KEY"),
            ) {
                (Ok(url), Ok(key)) => Some(PterodactylConfig {
                    panel_url: url,
                    api_key: key,
                }),
                _ => None,
            },
            features: Some(FeaturesConfig {
                auto_mod: Some(false),
                welcome_enabled: Some(true),
                economy_enabled: Some(false),
            }),
        })
    }
  }
      
