use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct InstalledDatabase {
    #[serde(default)]
    pub packages: HashMap<String, InstalledPackage>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub installed_at: DateTime<Utc>,
    pub install_mode: String,
    pub install_dir: String,
    pub files: Vec<String>,
    pub symlinks: Vec<String>,
    pub registry_source: String,
    #[serde(default)]
    pub checksums: HashMap<String, String>,
}
