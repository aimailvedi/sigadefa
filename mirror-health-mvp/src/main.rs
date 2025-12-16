mod app;
mod health;
mod llm;
mod persistence;
mod screenshot;

use app::{AppState, Config};
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> eframe::Result<()> {
    let config_path = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("config.toml");
    let config = load_or_create_config(&config_path);

    let base_dir = app_data_dir();
    fs::create_dir_all(base_dir.join("sessions")).ok();
    fs::create_dir_all(base_dir.join("screenshots")).ok();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mirror Health MVP",
        native_options,
        Box::new(move |cc| Box::new(AppState::new(cc, config.clone(), base_dir.clone()))),
    )
}

fn app_data_dir() -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        home.join(".mirror-health-mvp")
    } else {
        PathBuf::from(".mirror-health-mvp")
    }
}

fn load_or_create_config(path: &Path) -> Config {
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(cfg) = toml::from_str(&content) {
            return cfg;
        }
    }
    let default_cfg = Config::default();
    if let Ok(text) = toml::to_string_pretty(&default_cfg) {
        let _ = fs::write(path, text);
    }
    default_cfg
}
