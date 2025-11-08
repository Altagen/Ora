use anyhow::{Context, Result};
use chrono::Utc;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use crate::storage::paths::Paths;

pub struct AuditLogger;

impl AuditLogger {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub async fn log_install(
        &self,
        package: &str,
        version: &str,
        registry: &str,
        success: bool,
    ) -> Result<()> {
        let message = format!(
            "[{}] INSTALL package={} version={} registry={} success={}",
            Utc::now().to_rfc3339(),
            package,
            version,
            registry,
            success
        );
        self.write_log(&message).await
    }

    pub async fn log_uninstall(&self, package: &str, version: &str, success: bool) -> Result<()> {
        let message = format!(
            "[{}] UNINSTALL package={} version={} success={}",
            Utc::now().to_rfc3339(),
            package,
            version,
            success
        );
        self.write_log(&message).await
    }

    /// Reserved for future use when security event logging is implemented.
    #[allow(dead_code)]
    pub async fn log_security_event(&self, event_type: &str, details: &str) -> Result<()> {
        let message = format!(
            "[{}] SECURITY event={} details={}",
            Utc::now().to_rfc3339(),
            event_type,
            details
        );
        self.write_log(&message).await
    }

    async fn write_log(&self, message: &str) -> Result<()> {
        let log_file = Paths::audit_log_file()?;

        // Ensure parent directory exists
        if let Some(parent) = log_file.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .await
            .context("Failed to open audit log")?;

        file.write_all(message.as_bytes()).await?;
        file.write_all(b"\n").await?;
        file.sync_all().await?;

        Ok(())
    }
}
