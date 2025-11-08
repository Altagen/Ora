use crate::config::global::GlobalConfig;
use crate::config::repo::RepoConfig;
use crate::error::OraError;
use anyhow::Result;

pub struct SecurityWarningManager;

impl SecurityWarningManager {
    /// Check and display security warnings for a package
    pub fn check_and_warn(
        repo: &RepoConfig,
        allow_insecure: bool,
        global_config: &GlobalConfig,
    ) -> Result<()> {
        // Check if package allows insecure installation
        if !repo.security.allow_insecure {
            return Ok(());
        }

        // Check if user has suppressed warnings for this package
        if global_config.is_warning_suppressed(&repo.name) {
            log::debug!("Security warning suppressed for package '{}'", repo.name);
            return Ok(());
        }

        // Get warning message
        let message = if let Some(warnings) = &repo.security.warnings {
            if !warnings.enabled {
                return Ok(());
            }
            warnings
                .message
                .clone()
                .unwrap_or_else(|| Self::default_warning_message(&repo.name))
        } else {
            Self::default_warning_message(&repo.name)
        };

        // Display warning
        Self::display_warning(&repo.name, &message);

        // Check if user explicitly allowed insecure
        if !allow_insecure {
            eprintln!(
                "\n💡 To proceed anyway, use: ora install {} --allow-insecure",
                repo.name
            );
            eprintln!(
                "💡 To suppress this warning permanently: ora config suppress-warning {}",
                repo.name
            );
            return Err(OraError::InsecurePackage.into());
        }

        log::warn!("Proceeding with insecure installation (--allow-insecure flag used)");
        Ok(())
    }

    fn default_warning_message(package_name: &str) -> String {
        format!(
            "Package '{}' cannot verify checksums or signatures automatically. \
            This package may be insecure.",
            package_name
        )
    }

    fn display_warning(package_name: &str, message: &str) {
        eprintln!("\n⚠️  SECURITY WARNING for '{}' ⚠️", package_name);
        eprintln!("┌{}┐", "─".repeat(70));

        // Word wrap the message
        let words: Vec<&str> = message.split_whitespace().collect();
        let mut current_line = String::new();

        for word in words {
            if current_line.len() + word.len() + 1 > 66 {
                eprintln!("│ {:<68} │", current_line);
                current_line = String::new();
            }
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }

        if !current_line.is_empty() {
            eprintln!("│ {:<68} │", current_line);
        }

        eprintln!("└{}┘\n", "─".repeat(70));
    }
}

impl GlobalConfig {
    /// Check if warning is suppressed for a package
    pub fn is_warning_suppressed(&self, package_name: &str) -> bool {
        self.suppress_insecure_warnings
            .as_ref()
            .map(|list| list.contains(&package_name.to_string()))
            .unwrap_or(false)
    }
}
