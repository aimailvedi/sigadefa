# Mirror Health MVP

Desktop Rust application using eframe/egui with two LLM modes (Online via OpenAI HTTP, Local via Ollama), screenshot capture, session persistence, and health monitoring.

## Prerequisites
- Rust stable (edition 2021)
- Windows 11 x64 recommended
- Optional: [Ollama](https://ollama.com/download) running locally on `http://localhost:11434`
- Optional: OpenAI API key for online mode

## Configuration
The app reads `config.toml` in the project root. If it is missing, it will be created with defaults.

```
ollama_model = "phi3"
openai_model = "gpt-4o-mini"
openai_api_key = ""
history_limit = 12
request_timeout_sec = 25
min_free_disk_gb = 5
```

- Set `openai_api_key` for Online mode to work.
- Adjust `ollama_model` to a model available in your Ollama instance.
- `min_free_disk_gb` controls the health check threshold.

## Running
- On Windows you can double-click `start.bat` for a guided launch.
- Or run manually:
```
cargo run
```

## Using Online Mode (OpenAI)
1. Set `openai_api_key` in `config.toml`.
2. Choose **Online** in the top panel. Messages will be sent to `https://api.openai.com/v1/chat/completions` using the configured model.

## Using Local Mode (Ollama)
1. Install and start Ollama.
2. Ensure your desired model (e.g., `phi3`) is pulled: `ollama pull phi3`.
3. Select **Local** in the top panel to query Ollama via `http://localhost:11434/api/generate`.

## Features
- Chat UI with scrolling history and multiline input
- Session management: New, Save (JSON), Load (recent list), Clear
- Screenshot capture saved to `%USERPROFILE%\\.mirror-health-mvp\\screenshots`
- Health checks for app, screenshot success, Ollama reachability, OpenAI key presence, and disk space
- Status bar showing mode and last action/error
- Load dialog rendered as a modal window listing the 10 most recent sessions
