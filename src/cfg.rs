use serde::{Deserialize, Serialize};

/// Configuration for Syntect preprocessor,
#[derive(Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct SyntectConfig {
    /// Custom .tmTheme file path.
    pub custom_theme: Option<String>,
}

impl Default for SyntectConfig {
    fn default() -> SyntectConfig {
        SyntectConfig {
            custom_theme: None,
        }
    }
}

/// Extract configuration for katex preprocessor from `book_cfg`.
pub fn get_config(book_cfg: &mdbook::Config) -> SyntectConfig {
    let cfg = match book_cfg.get("preprocessor.syntect") {
        Some(raw) => raw.clone().try_into(),
        None => Ok(SyntectConfig::default()),
    };
    cfg.unwrap_or(SyntectConfig::default())
}
