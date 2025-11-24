use anyhow::{Context, Result};
use reqwest::{Client, Response};
use std::net::{IpAddr, ToSocketAddrs};
use std::time::Duration;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            // Use curl-like user agent for better compatibility with websites
            // that serve different content based on user agent
            .user_agent("curl/8.0.0")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client })
    }

    /// Validates URL for security before making requests
    fn validate_url(url: &str) -> Result<url::Url> {
        // Parse the URL
        let parsed = url::Url::parse(url).context(format!("Invalid URL: {}", url))?;

        // Only allow HTTP and HTTPS schemes
        match parsed.scheme() {
            "http" | "https" => {}
            scheme => {
                anyhow::bail!(
                    "Unsupported URL scheme '{}'. Only HTTP(S) is allowed for security reasons.",
                    scheme
                );
            }
        }

        // Check for localhost and private IP addresses (SSRF protection)
        if let Some(host) = parsed.host_str() {
            // Block localhost
            if host == "localhost" || host == "127.0.0.1" || host == "::1" {
                anyhow::bail!("Access to localhost is not allowed for security reasons");
            }

            // Try to parse as IP address and check if private
            if let Ok(ip) = host.parse::<IpAddr>() {
                if Self::is_private_ip(&ip) {
                    anyhow::bail!(
                        "Access to private IP addresses is not allowed for security reasons: {}",
                        ip
                    );
                }
            }

            // Check for link-local addresses
            if host.starts_with("169.254.") || host.starts_with("fe80:") {
                anyhow::bail!("Access to link-local addresses is not allowed for security reasons");
            }

            // Check for AWS metadata endpoint (common SSRF target)
            if host == "169.254.169.254" {
                anyhow::bail!(
                    "Access to cloud metadata endpoints is not allowed for security reasons"
                );
            }
        } else {
            anyhow::bail!("URL must have a valid host");
        }

        Ok(parsed)
    }

    /// Checks if an IP address is in a private range
    fn is_private_ip(ip: &IpAddr) -> bool {
        match ip {
            IpAddr::V4(ipv4) => {
                // 10.0.0.0/8
                ipv4.octets()[0] == 10
                    // 172.16.0.0/12
                    || (ipv4.octets()[0] == 172 && (ipv4.octets()[1] >= 16 && ipv4.octets()[1] <= 31))
                    // 192.168.0.0/16
                    || (ipv4.octets()[0] == 192 && ipv4.octets()[1] == 168)
                    // 127.0.0.0/8 (loopback)
                    || ipv4.octets()[0] == 127
            }
            IpAddr::V6(ipv6) => {
                // Check for private IPv6 ranges
                let segments = ipv6.segments();
                // fc00::/7 (Unique Local Addresses)
                (segments[0] & 0xfe00) == 0xfc00
                    // fe80::/10 (Link-Local)
                    || (segments[0] & 0xffc0) == 0xfe80
                    // ::1 (loopback)
                    || ipv6.is_loopback()
            }
        }
    }

    /// Validate DNS resolution to prevent DNS rebinding attacks
    ///
    /// **SECURITY**: DNS rebinding attack works by:
    /// 1. Attacker returns safe public IP on first DNS lookup
    /// 2. DNS TTL expires quickly (e.g., 1 second)
    /// 3. On second lookup, attacker returns private IP (e.g., 127.0.0.1)
    /// 4. Application makes request to private IP thinking it's safe
    ///
    /// This function re-resolves DNS just before making the request and validates
    /// that all resolved IPs are safe.
    fn validate_dns_resolution(url: &url::Url) -> Result<()> {
        // Load security config
        let config = crate::config::SecurityConfig::load().unwrap_or_default();

        // Check if DNS validation is enabled
        if !config.network.validate_dns_resolution {
            log::warn!("DNS validation is disabled - vulnerable to DNS rebinding attacks");
            return Ok(());
        }

        // Get hostname
        let host = url.host_str().context("URL must have a valid host")?;

        // If host is already an IP address, validate it directly
        if let Ok(ip) = host.parse::<IpAddr>() {
            if config.network.block_localhost && ip.is_loopback() {
                anyhow::bail!("Access to localhost IP is blocked: {}", ip);
            }
            if config.network.block_private_ips && Self::is_private_ip(&ip) {
                anyhow::bail!("Access to private IP is blocked: {}", ip);
            }
            return Ok(());
        }

        // Resolve hostname to IP addresses
        let port = url.port_or_known_default().unwrap_or(80);
        let socket_addr = format!("{}:{}", host, port);

        let resolved_ips: Vec<IpAddr> = match socket_addr.to_socket_addrs() {
            Ok(addrs) => addrs.map(|addr| addr.ip()).collect(),
            Err(e) => {
                log::warn!("DNS resolution failed for {}: {}", host, e);
                anyhow::bail!("DNS resolution failed for {}: {}", host, e);
            }
        };

        if resolved_ips.is_empty() {
            anyhow::bail!("DNS resolution returned no IP addresses for {}", host);
        }

        log::debug!("DNS resolved {} to IPs: {:?}", host, resolved_ips);

        // Validate each resolved IP
        for ip in resolved_ips {
            // Check for localhost
            if config.network.block_localhost && ip.is_loopback() {
                log::error!("❌ DNS rebinding attack detected!");
                log::error!("Hostname '{}' resolved to localhost IP: {}", host, ip);
                anyhow::bail!(
                    "DNS rebinding attack detected: hostname '{}' resolved to localhost IP {}",
                    host,
                    ip
                );
            }

            // Check for private IPs
            if config.network.block_private_ips && Self::is_private_ip(&ip) {
                log::error!("❌ DNS rebinding attack detected!");
                log::error!("Hostname '{}' resolved to private IP: {}", host, ip);
                anyhow::bail!(
                    "DNS rebinding attack detected: hostname '{}' resolved to private IP {}",
                    host,
                    ip
                );
            }

            // Check for link-local addresses
            if config.network.block_link_local && Self::is_link_local_ip(&ip) {
                log::error!("❌ DNS rebinding attack detected!");
                log::error!("Hostname '{}' resolved to link-local IP: {}", host, ip);
                anyhow::bail!(
                    "DNS rebinding attack detected: hostname '{}' resolved to link-local IP {}",
                    host,
                    ip
                );
            }

            // Check for cloud metadata endpoints
            if config.network.block_metadata_endpoints
                && matches!(ip, IpAddr::V4(ipv4) if ipv4.octets() == [169, 254, 169, 254])
            {
                log::error!("❌ SSRF attack attempt detected!");
                log::error!(
                    "Hostname '{}' resolved to cloud metadata endpoint: {}",
                    host,
                    ip
                );
                anyhow::bail!(
                    "SSRF attack detected: hostname '{}' resolved to cloud metadata endpoint {}",
                    host,
                    ip
                );
            }
        }

        Ok(())
    }

    /// Check if IP is link-local
    fn is_link_local_ip(ip: &IpAddr) -> bool {
        match ip {
            IpAddr::V4(ipv4) => {
                // 169.254.0.0/16
                ipv4.octets()[0] == 169 && ipv4.octets()[1] == 254
            }
            IpAddr::V6(ipv6) => {
                // fe80::/10
                let segments = ipv6.segments();
                (segments[0] & 0xffc0) == 0xfe80
            }
        }
    }

    pub async fn get(&self, url: &str) -> Result<Response> {
        // Validate URL before request
        let parsed_url = Self::validate_url(url)?;

        // SECURITY: DNS rebinding protection
        // Re-validate DNS resolution just before making the request
        Self::validate_dns_resolution(&parsed_url)?;

        self.client
            .get(url)
            .send()
            .await
            .context(format!("Failed to GET {}", url))
    }

    pub async fn download_file(&self, url: &str, dest: &std::path::Path) -> Result<()> {
        log::info!("Downloading {} to {:?}", url, dest);

        // Validate URL before download
        Self::validate_url(url)?;

        let response = self.get(url).await?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP error: {}", response.status());
        }

        // Check content length if available
        if let Some(content_length) = response.content_length() {
            use crate::config::security_limits::MAX_DOWNLOAD_SIZE;
            if content_length > MAX_DOWNLOAD_SIZE {
                anyhow::bail!(
                    "Download size ({} bytes) exceeds maximum allowed size ({} bytes)",
                    content_length,
                    MAX_DOWNLOAD_SIZE
                );
            }
        }

        let bytes = response
            .bytes()
            .await
            .context("Failed to read response bytes")?;

        tokio::fs::write(dest, &bytes)
            .await
            .context("Failed to write file")?;

        Ok(())
    }

    pub async fn get_text(&self, url: &str) -> Result<String> {
        // Validate URL
        Self::validate_url(url)?;

        let response = self.get(url).await?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP error: {}", response.status());
        }

        response
            .text()
            .await
            .context("Failed to read response text")
    }

    pub async fn get_json<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T> {
        // Validate URL
        Self::validate_url(url)?;

        let response = self.get(url).await?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP error: {}", response.status());
        }

        response
            .json::<T>()
            .await
            .context("Failed to parse JSON response")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_validation_allowed() {
        // Should allow HTTPS
        assert!(HttpClient::validate_url("https://github.com/user/repo").is_ok());

        // Should allow HTTP
        assert!(HttpClient::validate_url("http://example.com/file.tar.gz").is_ok());
    }

    #[test]
    fn test_url_validation_blocked_schemes() {
        // Should block file://
        assert!(HttpClient::validate_url("file:///etc/passwd").is_err());

        // Should block ftp://
        assert!(HttpClient::validate_url("ftp://example.com/file").is_err());

        // Should block data://
        assert!(HttpClient::validate_url("data:text/plain,hello").is_err());
    }

    #[test]
    fn test_url_validation_blocked_private_ips() {
        // Should block localhost
        assert!(HttpClient::validate_url("http://localhost/test").is_err());
        assert!(HttpClient::validate_url("http://127.0.0.1/test").is_err());

        // Should block private IPs
        assert!(HttpClient::validate_url("http://192.168.1.1/test").is_err());
        assert!(HttpClient::validate_url("http://10.0.0.1/test").is_err());
        assert!(HttpClient::validate_url("http://172.16.0.1/test").is_err());

        // Should block AWS metadata endpoint
        assert!(HttpClient::validate_url("http://169.254.169.254/latest/meta-data/").is_err());

        // Should block link-local
        assert!(HttpClient::validate_url("http://169.254.1.1/test").is_err());
    }

    #[test]
    fn test_private_ip_detection() {
        // Private ranges
        assert!(HttpClient::is_private_ip(
            &"10.0.0.1".parse().expect("valid IP")
        ));
        assert!(HttpClient::is_private_ip(
            &"192.168.1.1".parse().expect("valid IP")
        ));
        assert!(HttpClient::is_private_ip(
            &"172.16.0.1".parse().expect("valid IP")
        ));
        assert!(HttpClient::is_private_ip(
            &"127.0.0.1".parse().expect("valid IP")
        ));

        // Public IPs
        assert!(!HttpClient::is_private_ip(
            &"8.8.8.8".parse().expect("valid IP")
        ));
        assert!(!HttpClient::is_private_ip(
            &"1.1.1.1".parse().expect("valid IP")
        ));
    }
}
