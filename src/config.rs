use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

// global config
// each mode and pane is meant to be configurable
// this is the bootstrap, even if I don't make it before shipping xC
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub tagline: String,
    pub planning: PlanningConfig,
    pub review: ReviewConfig,
    pub focus: FocusConfig,
    pub theme: ThemeConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tagline: "Know what matters.".to_string(),
            planning: PlanningConfig::default(),
            review: ReviewConfig::default(),
            focus: FocusConfig::default(),
            theme: ThemeConfig::default(),
        }
    }
}

impl Config {
    pub fn load_or_create_default() -> Result<Self> {
        Self::load_or_create_at(default_config_path())
    }

    // this is a really cool thing I've found recently
    // it's basically: if a type can easily become the Path type, allow it as the argument
    pub fn load_or_create_at(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        if path.exists() {
            return Self::from_toml(&fs::read_to_string(path)?);
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config = Self::default();
        fs::write(path, toml::to_string_pretty(&config)?)?;

        Ok(config)
    }

    pub fn from_toml(input: &str) -> Result<Self> {
        Ok(toml::from_str(input)?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlanningConfig {
    pub block_style: String,
    pub loose_labels: Vec<String>,
}

impl Default for PlanningConfig {
    fn default() -> Self {
        Self {
            block_style: "mixed".to_string(),
            loose_labels: vec!["Now".to_string(), "Next".to_string(), "Later".to_string()],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ReviewConfig {
    pub scope: String,
    pub history_grouping: String,
}

impl Default for ReviewConfig {
    fn default() -> Self {
        Self {
            scope: "day".to_string(),
            history_grouping: "day".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct FocusConfig {
    pub default_minutes: u32,
    pub presets: Vec<u32>,
    pub mode: String,
}

impl Default for FocusConfig {
    fn default() -> Self {
        Self {
            default_minutes: 45,
            presets: vec![25, 45, 90],
            mode: "countdown".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeConfig {
    pub name: String,
    pub accent: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "dawn".to_string(),
            accent: "gold".to_string(),
        }
    }
}

fn default_config_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config/dawnline/config.toml")
}
