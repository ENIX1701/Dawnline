use dawnline::config::Config;
use dawnline::theme::DawnTheme;
use std::fs;
use uuid::Uuid;

fn temp_config_path() -> std::path::PathBuf {
    std::env::temp_dir()
        .join(format!("dawnline-config-{}", Uuid::now_v7()))
        .join("config.toml")
}

#[test]
fn creates_default_config_when_missing() -> color_eyre::Result<()> {
    let path = temp_config_path();

    let config = Config::load_or_create_at(&path)?;

    assert_eq!(config.tagline, "Know what matters.");
    assert_eq!(config.theme.name, "dawn");
    assert_eq!(config.focus.default_minutes, 45);
    assert!(path.exists());

    let _ = fs::remove_file(&path);
    if let Some(parent) = path.parent() {
        let _ = fs::remove_dir(parent);
    }

    Ok(())
}

#[test]
fn partial_config_uses_defaults() -> color_eyre::Result<()> {
    let config = Config::from_toml(
        r#"
tagline = "Cut the noise"

[theme]
name = "opal"
"#,
    )?;

    assert_eq!(config.tagline, "Cut the noise");
    assert_eq!(config.theme.name, "opal");
    assert_eq!(config.theme.accent, "gold");
    assert_eq!(config.review.scope, "day");
    assert_eq!(config.planning.loose_labels, vec!["Now", "Next", "Later"]);

    Ok(())
}

#[test]
fn theme_name_and_accent_are_selectable() {
    let dawn = DawnTheme::named("dawn");
    let opal = DawnTheme::named("opal");
    let mist = DawnTheme::named("mist");

    assert_ne!(dawn.accent, opal.accent);
    assert_ne!(opal.accent, mist.accent);

    let rose = DawnTheme::named("mist").with_accent_name("rose");

    assert_ne!(rose.accent, mist.accent);
}
