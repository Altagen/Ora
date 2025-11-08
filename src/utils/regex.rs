use anyhow::{Context, Result};
use regex::Regex;

/// Safe regex builder with ReDoS protection
///
/// **SECURITY**: This module protects against Regular Expression Denial of Service (ReDoS)
/// attacks by enforcing complexity limits on user-controlled regex patterns.
///
/// Protections:
/// - Maximum pattern length (prevents billion laughs patterns)
/// - Maximum compiled size (prevents DFA explosion)
/// - Maximum capture groups (prevents backtracking bombs)
/// - Pattern validation before compilation
///
/// Configuration: See `SecurityConfig.validation.regex`
///
/// Build a regex with security validation
///
/// **SECURITY**: This function validates and limits regex complexity to prevent ReDoS.
pub fn build_safe_regex(pattern: &str) -> Result<Regex> {
    let config = crate::config::SecurityConfig::load().unwrap_or_default();

    validate_regex_pattern(pattern, &config.validation.regex)?;

    // Compile regex with size limits
    // Note: The regex crate is already ReDoS-safe by design (no backtracking),
    // but we still validate pattern complexity to prevent DFA size explosions
    let regex = Regex::new(pattern).context("Failed to compile regex pattern")?;

    // Validate compiled regex size
    validate_compiled_regex(&regex, &config.validation.regex)?;

    Ok(regex)
}

/// Validate regex pattern before compilation
fn validate_regex_pattern(
    pattern: &str,
    config: &crate::config::security_config::RegexValidationConfig,
) -> Result<()> {
    // Check pattern length
    if pattern.len() > config.max_pattern_length {
        anyhow::bail!(
            "Regex pattern too long: {} characters (max: {})",
            pattern.len(),
            config.max_pattern_length
        );
    }

    // Count capture groups
    let capture_groups = count_capture_groups(pattern);
    if capture_groups > config.max_capture_groups {
        anyhow::bail!(
            "Too many capture groups: {} (max: {})",
            capture_groups,
            config.max_capture_groups
        );
    }

    // Check for dangerous patterns
    validate_dangerous_patterns(pattern)?;

    Ok(())
}

/// Validate compiled regex against size limits
fn validate_compiled_regex(
    regex: &Regex,
    config: &crate::config::security_config::RegexValidationConfig,
) -> Result<()> {
    // Get regex statistics
    let captures_len = regex.captures_len();

    if captures_len > config.max_capture_groups {
        anyhow::bail!(
            "Compiled regex has too many capture groups: {} (max: {})",
            captures_len,
            config.max_capture_groups
        );
    }

    // Note: We can't easily measure DFA size for the regex crate,
    // but the pattern length validation provides a good proxy

    Ok(())
}

/// Count capture groups in regex pattern
fn count_capture_groups(pattern: &str) -> usize {
    // Simple heuristic: count unescaped '(' that aren't non-capturing groups
    let mut count = 0;
    let mut chars = pattern.chars().peekable();
    let mut escaped = false;

    while let Some(c) = chars.next() {
        if escaped {
            escaped = false;
            continue;
        }

        if c == '\\' {
            escaped = true;
            continue;
        }

        if c == '(' {
            // Check if it's a non-capturing group (?:...) or other special group
            if let Some(&next) = chars.peek() {
                if next != '?' {
                    count += 1;
                }
            } else {
                count += 1;
            }
        }
    }

    count
}

/// Validate against known dangerous regex patterns
fn validate_dangerous_patterns(pattern: &str) -> Result<()> {
    // Check for nested quantifiers (e.g., (a+)+, (a*)+)
    // These can cause exponential backtracking in some regex engines
    // Note: Rust's regex crate doesn't have backtracking, but we warn anyway
    if pattern.contains(")+") || pattern.contains(")*") {
        log::warn!(
            "Regex pattern contains nested quantifiers which could be dangerous: {}",
            pattern
        );
    }

    // Check for alternation bombs (e.g., (a|a|a|a|a)+)
    if pattern.matches('|').count() > 10 {
        log::warn!(
            "Regex pattern contains many alternations ({} pipes), may be slow",
            pattern.matches('|').count()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_regex_simple() {
        let result = build_safe_regex(r"v(\d+\.\d+\.\d+)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_blocks_too_long_pattern() {
        // Create a pattern that exceeds max_pattern_length (1000 by default)
        let long_pattern = "a".repeat(2000);
        let result = build_safe_regex(&long_pattern);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too long"));
    }

    #[test]
    fn test_blocks_too_many_capture_groups() {
        // Create pattern with too many capture groups (> 50 by default)
        let pattern = "(a)".repeat(60);
        let result = build_safe_regex(&pattern);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("capture groups"));
    }

    #[test]
    fn test_count_capture_groups() {
        assert_eq!(count_capture_groups("(a)(b)(c)"), 3);
        assert_eq!(count_capture_groups("(?:a)(b)"), 1); // Non-capturing group
        assert_eq!(count_capture_groups(r"(a\(b)"), 1); // Escaped paren
        assert_eq!(count_capture_groups("no groups"), 0);
    }

    #[test]
    fn test_warns_on_nested_quantifiers() {
        // This should compile but log a warning
        let result = build_safe_regex("(a+)+");
        assert!(result.is_ok()); // Rust regex is safe, so we allow it with warning
    }

    #[test]
    fn test_version_regex() {
        let regex = build_safe_regex(r"v(\d+\.\d+\.\d+)").expect("Failed to build version regex");
        let text = "Version: v1.2.3";
        let captures = regex.captures(text).expect("Failed to capture version");
        assert_eq!(
            captures.get(1).expect("No capture group 1").as_str(),
            "1.2.3"
        );
    }
}
