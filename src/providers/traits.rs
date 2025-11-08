use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct Version {
    pub tag: String,
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub published_at: String,
    pub prerelease: bool,
}

#[async_trait]
pub trait VersionProvider: Send + Sync {
    async fn list_versions(&self) -> Result<Vec<Version>>;
    async fn get_download_url(&self, version: &str, os: &str, arch: &str) -> Result<String>;
    /// Reserved for future use when checksum URL fetching is implemented.
    #[allow(dead_code)]
    async fn get_checksum_url(&self, version: &str, os: &str, arch: &str)
        -> Result<Option<String>>;
    /// Reserved for future use when signature URL fetching is implemented.
    #[allow(dead_code)]
    async fn get_signature_url(
        &self,
        version: &str,
        os: &str,
        arch: &str,
    ) -> Result<Option<String>>;
}
