use chrono::Local;
use screenshots::Screen;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScreenshotError {
    #[error("Display not found")]
    NoDisplay,
    #[error("IO error: {0}")]
    Io(String),
}

pub fn capture_screenshot(base_dir: &Path) -> Result<String, ScreenshotError> {
    let screens = Screen::all().map_err(|_| ScreenshotError::NoDisplay)?;
    let screen = screens.first().ok_or(ScreenshotError::NoDisplay)?;

    let image = screen
        .capture()
        .map_err(|e| ScreenshotError::Io(e.to_string()))?;

    let dir = base_dir.join("screenshots");
    fs::create_dir_all(&dir).map_err(|e| ScreenshotError::Io(e.to_string()))?;
    let filename = format!("screenshot_{}.png", Local::now().format("%Y%m%d_%H%M%S"));
    let path = dir.join(filename);
    image
        .save(&path)
        .map_err(|e| ScreenshotError::Io(e.to_string()))?;
    Ok(path.to_string_lossy().to_string())
}
