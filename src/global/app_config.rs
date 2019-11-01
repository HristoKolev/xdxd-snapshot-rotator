use serde::{Serialize, Deserialize};

use super::prelude::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailConfig {
    pub notification_emails: Vec<String>,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_host: String,
    pub smtp_port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VmConfig {
    pub vm_name: String,
    pub min_snapshot_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub hostname: String,
    pub sentry_dsn: String,
    pub email_config: EmailConfig,
    pub snapshot_config: Option<HashMap<String, VmConfig>>,
}


pub fn read_config(file_path: &str) -> Result<AppConfig> {

    let json_content = ::std::fs::read_to_string(file_path)?;

    let materialized = ::serde_json::from_str(&json_content)?;

    Ok(materialized)
}
