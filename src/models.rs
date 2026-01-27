use crate::cli::ToolChoice;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Model alias mappings for OpenCode
static OPENCODE_ALIASES: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // Short aliases
    m.insert("opus", "anthropic/claude-opus-4-5");
    m.insert("sonnet", "anthropic/claude-sonnet-4-5");
    m.insert("haiku", "anthropic/claude-haiku-4-5");
    m.insert("codex", "openai/gpt-5.2-codex");
    m.insert("codex-mini", "openai/gpt-5.1-codex-mini");
    m.insert("codex-max", "openai/gpt-5.1-codex-max");
    // Providerless aliases (auto-prefix with provider)
    m.insert(
        "claude-3-5-haiku-20241022",
        "anthropic/claude-3-5-haiku-20241022",
    );
    m.insert(
        "claude-3-5-haiku-latest",
        "anthropic/claude-3-5-haiku-latest",
    );
    m.insert(
        "claude-3-5-sonnet-20240620",
        "anthropic/claude-3-5-sonnet-20240620",
    );
    m.insert(
        "claude-3-5-sonnet-20241022",
        "anthropic/claude-3-5-sonnet-20241022",
    );
    m.insert(
        "claude-3-7-sonnet-20250219",
        "anthropic/claude-3-7-sonnet-20250219",
    );
    m.insert(
        "claude-3-7-sonnet-latest",
        "anthropic/claude-3-7-sonnet-latest",
    );
    m.insert(
        "claude-3-haiku-20240307",
        "anthropic/claude-3-haiku-20240307",
    );
    m.insert(
        "claude-3-opus-20240229",
        "anthropic/claude-3-opus-20240229",
    );
    m.insert(
        "claude-3-sonnet-20240229",
        "anthropic/claude-3-sonnet-20240229",
    );
    m.insert("claude-haiku-4-5", "anthropic/claude-haiku-4-5");
    m.insert(
        "claude-haiku-4-5-20251001",
        "anthropic/claude-haiku-4-5-20251001",
    );
    m.insert("claude-opus-4-0", "anthropic/claude-opus-4-0");
    m.insert("claude-opus-4-1", "anthropic/claude-opus-4-1");
    m.insert(
        "claude-opus-4-1-20250805",
        "anthropic/claude-opus-4-1-20250805",
    );
    m.insert("claude-opus-4-20250514", "anthropic/claude-opus-4-20250514");
    m.insert("claude-opus-4-5", "anthropic/claude-opus-4-5");
    m.insert(
        "claude-opus-4-5-20251101",
        "anthropic/claude-opus-4-5-20251101",
    );
    m.insert("claude-sonnet-4-0", "anthropic/claude-sonnet-4-0");
    m.insert(
        "claude-sonnet-4-20250514",
        "anthropic/claude-sonnet-4-20250514",
    );
    m.insert("claude-sonnet-4-5", "anthropic/claude-sonnet-4-5");
    m.insert(
        "claude-sonnet-4-5-20250929",
        "anthropic/claude-sonnet-4-5-20250929",
    );
    // OpenAI providerless aliases
    m.insert("gpt-5.1-codex-max", "openai/gpt-5.1-codex-max");
    m.insert("gpt-5.1-codex-mini", "openai/gpt-5.1-codex-mini");
    m.insert("gpt-5.2", "openai/gpt-5.2");
    m.insert("gpt-5.2-codex", "openai/gpt-5.2-codex");
    m
});

/// Model alias mappings for Codex tool
static CODEX_ALIASES: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("codex", "gpt-5.2-codex");
    m.insert("codex-mini", "gpt-5.1-codex-mini");
    m.insert("codex-max", "gpt-5.1-codex-max");
    m
});

/// Resolve a model alias to its full name for the given tool
pub fn resolve_model(tool: &ToolChoice, model: &str) -> String {
    match tool {
        ToolChoice::OpenCode => OPENCODE_ALIASES
            .get(model)
            .map(|s| s.to_string())
            .unwrap_or_else(|| model.to_string()),
        ToolChoice::Codex => CODEX_ALIASES
            .get(model)
            .map(|s| s.to_string())
            .unwrap_or_else(|| model.to_string()),
        ToolChoice::Claude => model.to_string(),
    }
}

/// Get the default model for a tool (if any)
pub fn default_model(tool: &ToolChoice) -> Option<String> {
    match tool {
        ToolChoice::OpenCode => Some("anthropic/claude-opus-4-5".to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_model_short_alias_opus() {
        assert_eq!(
            resolve_model(&ToolChoice::OpenCode, "opus"),
            "anthropic/claude-opus-4-5"
        );
    }

    #[test]
    fn test_resolve_model_short_alias_sonnet() {
        assert_eq!(
            resolve_model(&ToolChoice::OpenCode, "sonnet"),
            "anthropic/claude-sonnet-4-5"
        );
    }

    #[test]
    fn test_resolve_model_short_alias_haiku() {
        assert_eq!(
            resolve_model(&ToolChoice::OpenCode, "haiku"),
            "anthropic/claude-haiku-4-5"
        );
    }

    #[test]
    fn test_resolve_model_providerless_claude() {
        assert_eq!(
            resolve_model(&ToolChoice::OpenCode, "claude-3-5-sonnet-20241022"),
            "anthropic/claude-3-5-sonnet-20241022"
        );
    }

    #[test]
    fn test_resolve_model_providerless_openai() {
        assert_eq!(
            resolve_model(&ToolChoice::OpenCode, "gpt-5.2-codex"),
            "openai/gpt-5.2-codex"
        );
    }

    #[test]
    fn test_resolve_model_passthrough_already_qualified() {
        assert_eq!(
            resolve_model(&ToolChoice::OpenCode, "custom/model"),
            "custom/model"
        );
    }

    #[test]
    fn test_resolve_model_passthrough_unknown() {
        assert_eq!(
            resolve_model(&ToolChoice::OpenCode, "unknown-model"),
            "unknown-model"
        );
    }

    #[test]
    fn test_resolve_model_codex_short_alias() {
        assert_eq!(
            resolve_model(&ToolChoice::Codex, "codex"),
            "gpt-5.2-codex"
        );
    }

    #[test]
    fn test_resolve_model_codex_mini() {
        assert_eq!(
            resolve_model(&ToolChoice::Codex, "codex-mini"),
            "gpt-5.1-codex-mini"
        );
    }

    #[test]
    fn test_resolve_model_codex_passthrough() {
        assert_eq!(
            resolve_model(&ToolChoice::Codex, "o1-preview"),
            "o1-preview"
        );
    }

    #[test]
    fn test_resolve_model_claude_passthrough() {
        // Claude tool doesn't have aliases - everything passes through
        assert_eq!(
            resolve_model(&ToolChoice::Claude, "claude-3-opus-20240229"),
            "claude-3-opus-20240229"
        );
    }

    #[test]
    fn test_default_model_opencode() {
        assert_eq!(
            default_model(&ToolChoice::OpenCode),
            Some("anthropic/claude-opus-4-5".to_string())
        );
    }

    #[test]
    fn test_default_model_claude() {
        assert_eq!(default_model(&ToolChoice::Claude), None);
    }

    #[test]
    fn test_default_model_codex() {
        assert_eq!(default_model(&ToolChoice::Codex), None);
    }
}
