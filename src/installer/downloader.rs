use anyhow::Result;
use std::path::Path;

use crate::utils::http::HttpClient;

pub struct Downloader {
    client: HttpClient,
}

impl Downloader {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: HttpClient::new()?,
        })
    }

    pub async fn download(&self, url: &str, dest: &Path) -> Result<()> {
        log::debug!("Downloading from {} to {:?}", url, dest);

        // Ensure parent directory exists
        if let Some(parent) = dest.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        self.client.download_file(url, dest).await?;

        log::debug!("Download completed");
        Ok(())
    }

    pub async fn download_text(&self, url: &str) -> Result<String> {
        log::debug!("Downloading text from {}", url);
        self.client.get_text(url).await
    }
}
