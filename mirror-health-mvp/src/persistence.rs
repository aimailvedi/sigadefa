use crate::app::{Message, Mode};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub display_name: String,
    pub path: PathBuf,
    pub messages: Vec<Message>,
    pub mode: Mode,
    pub timestamp: DateTime<Local>,
}

pub fn save_session(base_dir: &Path, messages: &[Message], mode: Mode) -> anyhow::Result<String> {
    let session_dir = base_dir.join("sessions");
    fs::create_dir_all(&session_dir)?;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("session_{}.json", timestamp);
    let path = session_dir.join(filename);
    let data = SessionFile {
        mode,
        messages: messages.to_vec(),
        timestamp: Local::now(),
    };
    let text = serde_json::to_string_pretty(&data)?;
    let mut file = File::create(&path)?;
    file.write_all(text.as_bytes())?;
    Ok(path.to_string_lossy().to_string())
}

pub fn load_session(path: &Path) -> anyhow::Result<SessionFile> {
    let content = fs::read_to_string(path)?;
    let data: SessionFile = serde_json::from_str(&content)?;
    Ok(data)
}

pub fn list_sessions(base_dir: &Path) -> Vec<SessionData> {
    let session_dir = base_dir.join("sessions");
    let mut sessions = Vec::new();
    if let Ok(entries) = fs::read_dir(session_dir) {
        for entry in entries.flatten() {
            if entry.metadata().is_ok() {
                let path = entry.path();
                if path.is_file() {
                    if let Ok(data) = load_session(&path) {
                        let name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("session")
                            .to_string();
                        sessions.push(SessionData {
                            display_name: name,
                            path: path.clone(),
                            messages: data.messages,
                            mode: data.mode,
                            timestamp: data.timestamp,
                        });
                    }
                }
            }
        }
    }
    sessions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    sessions.truncate(10);
    sessions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionFile {
    pub mode: Mode,
    pub messages: Vec<Message>,
    pub timestamp: DateTime<Local>,
}
