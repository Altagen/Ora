use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocalMetadata {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub binaries: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl LocalMetadata {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.is_empty() {
            anyhow::bail!("Package name cannot be empty");
        }
        if self.version.is_empty() {
            anyhow::bail!("Package version cannot be empty");
        }
        if self.binaries.is_empty() {
            anyhow::bail!("At least one binary must be specified");
        }
        Ok(())
    }
}
