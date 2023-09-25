use crate::model::constants::SIGNING_KEY;
use std::path::Path;
use anyhow::Context;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceConfig {
    pub service_id: String,
}

/// Reads a file into a string
pub fn read_file(file: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(file)
        .with_context(|| format!("Failed to read contents of file '{}'", file))
}
