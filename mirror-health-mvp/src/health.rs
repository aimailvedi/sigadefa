use crate::app::Config;
use reqwest::Client;
use serde::Serialize;
use sysinfo::Disks;

#[derive(Debug, Clone, Serialize)]
pub struct HealthComponent {
    pub name: String,
    pub ok: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthStatus {
    pub score: u8,
    pub components: Vec<HealthComponent>,
}

pub async fn run_health_checks(config: &Config, screenshot_ok: bool) -> HealthStatus {
    let mut components = Vec::new();

    components.push(HealthComponent {
        name: "App".to_string(),
        ok: true,
        detail: "UI running".to_string(),
    });

    components.push(HealthComponent {
        name: "Screenshot".to_string(),
        ok: screenshot_ok,
        detail: if screenshot_ok {
            "Last screenshot succeeded".to_string()
        } else {
            "No successful screenshot yet".to_string()
        },
    });

    let ollama_ok = check_ollama(config).await;
    components.push(HealthComponent {
        name: "OllamaApi".to_string(),
        ok: ollama_ok,
        detail: if ollama_ok {
            "Ollama reachable".to_string()
        } else {
            "Ollama not reachable".to_string()
        },
    });

    let openai_ok = !config.openai_api_key.trim().is_empty();
    components.push(HealthComponent {
        name: "OpenAiConfigured".to_string(),
        ok: openai_ok,
        detail: if openai_ok {
            "API key configured".to_string()
        } else {
            "No API key".to_string()
        },
    });

    let disk_ok = check_disk_space(config.min_free_disk_gb);
    components.push(HealthComponent {
        name: "DiskSpaceOk".to_string(),
        ok: disk_ok,
        detail: if disk_ok {
            format!("Free space >= {} GB", config.min_free_disk_gb)
        } else {
            format!("Less than {} GB free", config.min_free_disk_gb)
        },
    });

    let mut score: i32 = 100;
    for comp in &components {
        if !comp.ok {
            score -= 20;
        }
    }
    if score < 0 {
        score = 0;
    }
    if score > 100 {
        score = 100;
    }

    HealthStatus {
        score: score as u8,
        components,
    }
}

async fn check_ollama(config: &Config) -> bool {
    let client = match Client::builder()
        .timeout(std::time::Duration::from_secs(config.request_timeout_sec))
        .build()
    {
        Ok(c) => c,
        Err(_) => return false,
    };
    let url = "http://localhost:11434/api/version";
    match client.get(url).send().await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

fn check_disk_space(min_free_gb: u64) -> bool {
    let disks = Disks::new_with_refreshed_list();
    let min_bytes = min_free_gb * 1024 * 1024 * 1024;
    for disk in disks.list() {
        if disk.available_space() >= min_bytes {
            return true;
        }
    }
    false
}
