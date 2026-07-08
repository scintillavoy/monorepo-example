use std::{collections::HashMap, env, fs};

use config::{Config, File};
use serde::Deserialize;
use vaultrs::{
    client::{VaultClient, VaultClientSettingsBuilder},
    kv1,
};

#[derive(Deserialize)]
pub struct ApiSecret {
    pub mysql_password: String,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

impl ApiSecret {
    pub async fn new(vault_address: &str, vault_path: &str) -> anyhow::Result<Self> {
        const PREFIX: &str = "api_";
        if let Ok(secret_path) = env::var("SECRET_PATH") {
            let config = Config::builder()
                .add_source(File::with_name(&secret_path))
                .build()?;
            let raw: HashMap<String, String> = config.try_deserialize()?;
            let filtered = extract_component(raw, PREFIX);
            let secret = serde_json::from_value(serde_json::to_value(filtered)?)?;
            return Ok(secret);
        }
        let vault_token_path = env::var("VAULT_TOKEN_PATH")?;
        let vault_token = fs::read_to_string(vault_token_path)?;
        let client = VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(vault_address)
                .token(vault_token)
                .build()?,
        )?;
        let raw: HashMap<String, String> = kv1::get(&client, "secret", vault_path).await?;
        let filtered = extract_component(raw, PREFIX);
        let secret = serde_json::from_value(serde_json::to_value(filtered)?)?;
        Ok(secret)
    }
}

fn extract_component(raw: HashMap<String, String>, prefix: &str) -> HashMap<String, String> {
    raw.into_iter()
        .filter_map(|(k, v)| {
            k.strip_prefix(prefix)
                .map(|stripped| (stripped.to_string(), v))
        })
        .collect()
}
