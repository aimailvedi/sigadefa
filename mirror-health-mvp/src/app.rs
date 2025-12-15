use crate::health::{self, HealthStatus};
use crate::llm::{send_ollama, send_openai};
use crate::persistence::{list_sessions, load_session, save_session, SessionData};
use crate::screenshot::capture_screenshot;
use chrono::{DateTime, Local};
use eframe::egui::{self, Align, Color32, Label, Layout, RichText, ScrollArea};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use tokio::runtime::Runtime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ollama_model: String,
    pub openai_model: String,
    pub openai_api_key: String,
    pub history_limit: usize,
    pub request_timeout_sec: u64,
    pub min_free_disk_gb: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ollama_model: "phi3".to_string(),
            openai_model: "gpt-4o-mini".to_string(),
            openai_api_key: String::new(),
            history_limit: 12,
            request_timeout_sec: 25,
            min_free_disk_gb: 5,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Mode {
    Online,
    Local,
}

impl Mode {
    pub fn label(&self) -> &'static str {
        match self {
            Mode::Online => "Online",
            Mode::Local => "Local",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub timestamp: DateTime<Local>,
}

impl Message {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
            timestamp: Local::now(),
        }
    }
}

pub enum UiEvent {
    Chat(Message),
    Health(HealthStatus),
    LastError(String),
}

pub struct AppState {
    pub messages: Vec<Message>,
    pub input_text: String,
    pub mode: Mode,
    pub config: Config,
    pub thinking: bool,
    pub last_action: String,
    pub last_error: Option<String>,
    pub last_health: Option<HealthStatus>,
    pub show_load_panel: bool,
    pub available_sessions: Vec<SessionData>,
    pub last_screenshot_path: Option<String>,
    runtime: Arc<Runtime>,
    sender: Sender<UiEvent>,
    receiver: Receiver<UiEvent>,
    base_dir: PathBuf,
}

impl AppState {
    pub fn new(_cc: &eframe::CreationContext<'_>, config: Config, base_dir: PathBuf) -> Self {
        let runtime = Runtime::new().expect("Failed to start tokio runtime");
        let (sender, receiver) = channel();
        let mut app = Self {
            messages: Vec::new(),
            input_text: String::new(),
            mode: Mode::Online,
            config,
            thinking: false,
            last_action: "Ready".to_string(),
            last_error: None,
            last_health: None,
            show_load_panel: false,
            available_sessions: Vec::new(),
            last_screenshot_path: None,
            runtime: Arc::new(runtime),
            sender,
            receiver,
            base_dir,
        };
        app.start_health_check(false);
        app
    }

    pub fn update_ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(event) = self.receiver.try_recv() {
            match event {
                UiEvent::Chat(msg) => {
                    self.messages.push(msg);
                    self.thinking = false;
                }
                UiEvent::Health(status) => {
                    self.last_health = Some(status);
                    ctx.request_repaint();
                }
                UiEvent::LastError(err) => {
                    self.last_error = Some(err.clone());
                    self.last_action = err;
                    self.thinking = false;
                }
            }
        }
        self.render_top_panel(ctx);
        self.render_left_panel(ctx);
        self.render_chat(ctx);
        self.render_input(ctx);
        self.render_status_bar(ctx);
    }

    fn render_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let health_score = self.last_health.as_ref().map(|h| h.score).unwrap_or(0);
                let color = if health_score > 80 {
                    Color32::GREEN
                } else if health_score > 40 {
                    Color32::YELLOW
                } else {
                    Color32::RED
                };
                ui.colored_label(color, format!("Health Score: {}", health_score));

                ui.separator();
                ui.label("Mode:");
                ui.selectable_value(&mut self.mode, Mode::Online, "Online");
                ui.selectable_value(&mut self.mode, Mode::Local, "Local");

                if ui.button("Health Check Now").clicked() {
                    self.start_health_check(true);
                }
            });

            if let Some(health) = &self.last_health {
                ui.horizontal_wrapped(|ui| {
                    for comp in &health.components {
                        let text = format!("{}: {}", comp.name, comp.detail);
                        let color = if comp.ok {
                            Color32::GREEN
                        } else {
                            Color32::RED
                        };
                        ui.colored_label(color, text);
                    }
                });
            }
        });
    }

    fn render_left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Sessions");
            if ui.button("New Session").clicked() {
                self.messages.clear();
                self.input_text.clear();
                self.last_action = "Started new session".to_string();
            }
            if ui.button("Save Session").clicked() {
                match save_session(&self.base_dir, &self.messages, self.mode) {
                    Ok(path) => {
                        self.last_action = format!("Saved session: {}", path);
                    }
                    Err(e) => {
                        let msg = format!("Failed to save session: {}", e);
                        self.last_error = Some(msg.clone());
                        self.messages.push(Message::new(Role::System, msg.clone()));
                        self.last_action = msg;
                    }
                }
            }
            if ui.button("Load Session").clicked() {
                self.available_sessions = list_sessions(&self.base_dir);
                self.show_load_panel = true;
            }
            if ui.button("Clear Chat").clicked() {
                self.messages.clear();
                self.input_text.clear();
                self.last_action = "Cleared chat".to_string();
            }
            if ui.button("Screenshot").clicked() {
                match capture_screenshot(&self.base_dir) {
                    Ok(path) => {
                        self.last_screenshot_path = Some(path.clone());
                        let msg = format!("Screenshot captured: {}", path);
                        self.messages.push(Message::new(Role::System, msg.clone()));
                        self.last_action = msg;
                        self.start_health_check(true);
                    }
                    Err(e) => {
                        let msg = format!("Screenshot failed: {}", e);
                        self.messages.push(Message::new(Role::System, msg.clone()));
                        self.last_error = Some(msg.clone());
                        self.last_action = msg;
                        self.start_health_check(true);
                    }
                }
            }

            if self.show_load_panel {
                ui.separator();
                ui.label("Recent Sessions:");
                for session in &self.available_sessions {
                    ui.horizontal(|ui| {
                        ui.label(&session.display_name);
                        if ui.button("Load").clicked() {
                            if let Ok(data) = load_session(&session.path) {
                                self.messages = data.messages;
                                self.mode = data.mode;
                                self.last_action = format!("Loaded {}", session.display_name);
                            } else {
                                let msg = "Failed to load session".to_string();
                                self.messages.push(Message::new(Role::System, msg.clone()));
                                self.last_error = Some(msg.clone());
                                self.last_action = msg;
                            }
                            self.show_load_panel = false;
                        }
                    });
                }
            }
        });
    }

    fn render_chat(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for message in &self.messages {
                        let color = match message.role {
                            Role::User => Color32::from_rgb(200, 230, 255),
                            Role::Assistant => Color32::from_rgb(200, 255, 200),
                            Role::System => Color32::from_rgb(240, 200, 200),
                        };
                        let label =
                            format!("{}: {}", self.role_label(&message.role), message.content);
                        ui.colored_label(color, label);
                    }
                    if self.thinking {
                        ui.add(Label::new(
                            RichText::new("Assistant is thinking...\nPlease wait.").italics(),
                        ));
                    }
                });
        });
    }

    fn render_input(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("input_panel").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                ui.vertical(|ui| {
                    ui.label("Enter message:");
                    ui.add_sized(
                        [ui.available_width(), 80.0],
                        egui::TextEdit::multiline(&mut self.input_text).hint_text("Type here..."),
                    );
                    ui.horizontal(|ui| {
                        if ui.button("Send").clicked() {
                            self.handle_send();
                        }
                        if self.mode == Mode::Online {
                            ui.label("Online mode uses OpenAI");
                        } else {
                            ui.label("Local mode uses Ollama");
                        }
                    });
                });
            });
        });
    }

    fn render_status_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Current mode: {}", self.mode.label()));
                ui.separator();
                if let Some(err) = &self.last_error {
                    ui.colored_label(Color32::RED, format!("Last error: {}", err));
                } else {
                    ui.label(format!("Last action: {}", self.last_action));
                }
                if let Some(path) = &self.last_screenshot_path {
                    ui.separator();
                    ui.label(format!("Last screenshot: {}", path));
                }
            });
        });
    }

    fn handle_send(&mut self) {
        if self.input_text.trim().is_empty() || self.thinking {
            return;
        }
        let content = self.input_text.trim().to_string();
        self.input_text.clear();
        let user_msg = Message::new(Role::User, content);
        self.messages.push(user_msg.clone());
        self.thinking = true;
        self.last_action = "Sent message".to_string();

        let history = self.recent_history();
        let config = self.config.clone();
        let sender = self.sender.clone();
        let mode = self.mode;
        let runtime = self.runtime.clone();

        runtime.spawn(async move {
            let result = match mode {
                Mode::Local => send_ollama(&config, history).await,
                Mode::Online => send_openai(&config, history).await,
            };
            match result {
                Ok(resp) => {
                    let _ = sender.send(UiEvent::Chat(Message::new(Role::Assistant, resp)));
                }
                Err(e) => {
                    let msg = format!("LLM error: {}", e);
                    let _ = sender.send(UiEvent::Chat(Message::new(Role::System, msg.clone())));
                    let _ = sender.send(UiEvent::LastError(msg));
                }
            }
        });
    }

    fn start_health_check(&mut self, manual: bool) {
        let sender = self.sender.clone();
        let cfg = self.config.clone();
        let runtime = self.runtime.clone();
        let screenshot_ok = self.last_screenshot_path.is_some();
        self.last_action = if manual {
            "Running health check".to_string()
        } else {
            "Initialized health check".to_string()
        };
        runtime.spawn(async move {
            let status = health::run_health_checks(&cfg, screenshot_ok).await;
            let _ = sender.send(UiEvent::Health(status));
        });
    }

    fn recent_history(&self) -> Vec<Message> {
        let limit = self.config.history_limit;
        self.messages
            .iter()
            .rev()
            .take(limit)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    fn role_label(&self, role: &Role) -> &str {
        match role {
            Role::User => "USER",
            Role::Assistant => "ASSISTANT",
            Role::System => "SYSTEM",
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update_ui(ctx, frame);
    }
}
