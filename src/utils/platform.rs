use std::collections::HashMap;

pub struct Platform {
    pub os: String,
    pub arch: String,
}

impl Platform {
    pub fn detect() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
        }
    }

    pub fn map_os(&self, mapping: &HashMap<String, String>) -> String {
        mapping
            .get(&self.os)
            .cloned()
            .unwrap_or_else(|| self.os.clone())
    }

    pub fn map_arch(&self, mapping: &HashMap<String, String>) -> String {
        mapping
            .get(&self.arch)
            .cloned()
            .unwrap_or_else(|| self.arch.clone())
    }

    /// Reserved for future use when platform key is needed.
    #[allow(dead_code)]
    pub fn platform_key(&self) -> String {
        format!("{}_{}", self.os, self.arch)
    }

    /// Reserved for future use when platform support checking is needed.
    #[allow(dead_code)]
    pub fn is_supported() -> bool {
        matches!(
            (std::env::consts::OS, std::env::consts::ARCH),
            ("linux", "x86_64") | ("linux", "aarch64") | ("macos", "x86_64") | ("macos", "aarch64")
        )
    }
}

// Default mappings for common cases
pub fn default_os_mapping() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("macos".to_string(), "darwin".to_string());
    map.insert("linux".to_string(), "linux".to_string());
    map
}

pub fn default_arch_mapping() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("x86_64".to_string(), "amd64".to_string());
    map.insert("aarch64".to_string(), "arm64".to_string());
    map
}
