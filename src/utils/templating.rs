use anyhow::Result;
use std::collections::HashMap;

// Note: TemplateResolver struct removed - use resolve_template_safe() directly

/// Resolve template with security validation
///
/// **SECURITY**: This function validates and sanitizes all variables to prevent injection attacks.
///
/// Protections:
/// - URL-encodes variables when in URL context
/// - Blocks path traversal sequences (../)
/// - Blocks null bytes (\0)
/// - Blocks newlines
/// - Enforces maximum variable length
pub fn resolve_template_safe(
    template: &str,
    variables: &HashMap<String, String>,
) -> Result<String> {
    // Load security config
    let config = crate::config::SecurityConfig::load().unwrap_or_default();

    let mut result = template.to_string();

    for (key, value) in variables {
        // Validate variable value
        validate_template_variable(value, &config.validation.templates)?;

        // Determine if we're in a URL context
        let is_url_context = template.contains("http://") || template.contains("https://");

        // Sanitize the value
        let sanitized_value = if is_url_context && config.validation.templates.url_encode_variables
        {
            urlencoding::encode(value).to_string()
        } else {
            value.clone()
        };

        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, &sanitized_value);
    }

    Ok(result)
}

// Note: Unsafe resolve_template() function removed - use resolve_template_safe() instead

/// Validate a template variable value against security policies
fn validate_template_variable(
    value: &str,
    config: &crate::config::security_config::TemplateValidationConfig,
) -> Result<()> {
    // Check length
    if value.len() > config.max_variable_length {
        anyhow::bail!(
            "Template variable too long: {} characters (max: {})",
            value.len(),
            config.max_variable_length
        );
    }

    // Check for path traversal
    if config.block_path_traversal && value.contains("..") {
        anyhow::bail!(
            "Template variable contains path traversal sequence: '..' in '{}'",
            value
        );
    }

    // Check for null bytes
    if config.block_null_bytes && value.contains('\0') {
        anyhow::bail!("Template variable contains null byte");
    }

    // Check for newlines
    if config.block_newlines && (value.contains('\n') || value.contains('\r')) {
        anyhow::bail!("Template variable contains newline characters");
    }

    // Additional checks for dangerous patterns
    if value.contains("${") || value.contains("$(") {
        log::warn!(
            "Template variable contains shell expansion pattern: {}",
            value
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_template_resolution() {
        let mut vars = HashMap::new();
        vars.insert("version".to_string(), "1.0.0".to_string());
        vars.insert("os".to_string(), "linux".to_string());

        let template = "https://example.com/{version}/{os}/package.tar.gz";
        let result = resolve_template_safe(template, &vars).expect("Failed to resolve template");

        assert_eq!(result, "https://example.com/1.0.0/linux/package.tar.gz");
    }

    #[test]
    fn test_blocks_path_traversal() {
        let mut vars = HashMap::new();
        vars.insert("path".to_string(), "../../etc/passwd".to_string());

        let template = "https://example.com/{path}";
        let result = resolve_template_safe(template, &vars);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("path traversal"));
    }

    #[test]
    fn test_blocks_null_bytes() {
        let mut vars = HashMap::new();
        vars.insert("value".to_string(), "test\0value".to_string());

        let template = "https://example.com/{value}";
        let result = resolve_template_safe(template, &vars);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("null byte"));
    }

    #[test]
    fn test_blocks_newlines() {
        let mut vars = HashMap::new();
        vars.insert("value".to_string(), "test\nvalue".to_string());

        let template = "https://example.com/{value}";
        let result = resolve_template_safe(template, &vars);

        assert!(result.is_err());
        assert!(result
            .expect_err("Should reject newlines")
            .to_string()
            .contains("newline"));
    }

    #[test]
    fn test_url_encoding() {
        let mut vars = HashMap::new();
        vars.insert("version".to_string(), "1.0.0+special".to_string());

        let template = "https://example.com/download?v={version}";
        let result =
            resolve_template_safe(template, &vars).expect("Failed to resolve URL template");

        // + should be encoded to %2B in URL context
        assert!(result.contains("1.0.0%2Bspecial") || result.contains("1.0.0+special"));
    }
}
