# Универсальный промт для Codex: Rust Vision-Voice Assistant

## Назначение
Быстрый запуск MVP голосового ассистента с компьютерным зрением на Rust в формате «цифрового зеркала». Промт фиксирует архитектуру, ограничения и этапы разработки, чтобы Codex мог выдавать предсказуемый и компактный код.

## Структура проекта (сигадеф-принципы)
```
rust-vision-mirror/
├── core/           # Только типы данных и состояние
├── ui/             # Только отрисовка (egui)
├── ops/            # Чистые функции операций
├── app.rs          # State machine (< 200 строк)
└── main.rs         # Точка входа (< 50 строк)
```

## Зависимости (Cargo.toml)
```
eframe = "0.24"
egui = "0.24"
tokio = { version = "1", features = ["full"] }
reqwest = "0.11"
image = "0.24"
screenshots = "0.5"      # или windows = "0.51" для WinAPI
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1.0"
```

## Ключевые концепции
- Зеркальная метафора: ассистент **не действует сам**, он отражает и комментирует увиденное/услышанное.
- Режимы восприятия: 👂 Hear (голос), 👁️ See (скриншот), 💭 Think (LLM).
- Фрактальная архитектура: единая схема команд пригодна для голоса, кнопок UI и нод Houdini.

## Ограничения качества
- Каждая функция ≤ 10 строк, без скрытой магии.
- Нет глобального состояния, все зависимости явные.
- Тайм-ауты: захват экрана < 100 мс, голос < 500 мс, LLM ≤ 30 с.
- Юнит-тесты для core, интеграционные тесты для API, мок внешних вызовов.

## План MVP (по этапам)
1) **Базовый чат**: окно с историей и вводом, кнопка «Отправить», `AppState { messages, input }`.
2) **Скриншоты**: кнопка «📷 Скриншот», захват экрана → `Option<Vec<u8>>`, миниатюра в UI, пока без LLM.
3) **Голос**: ключевая фраза «Зеркало, ...», запись через системный API или `cpal + vosk`, транскрипция Whisper/API → подстановка в поле ввода.
4) **LLM**: режимы Local (Ollama) и Online (OpenAI/Claude), отправка текста + base64-скриншота, обработка ошибок, ответ в чате.
5) **Vision**: GPT-4V или LLaVA через Ollama, контекстные подсказки по изображению.

## Базовые типы (core/types.rs)
```
#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub messages: Vec<Message>,
    pub input_buffer: String,
    pub mode: Mode,                  // Local | Online
    pub current_screenshot: Option<Vec<u8>>,
    pub is_listening: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    User(String, Option<Vec<u8>>),   // текст + опциональный скриншот
    Assistant(String),
    System(String),                  // «Слушаю...» и т.п.
}
```

## Машина состояний (app.rs, < 200 строк)
```
impl AppState {
    pub fn handle_ui_event(&mut self, event: UiEvent) {
        match event {
            UiEvent::InputChanged(text) => self.input_buffer = text,
            UiEvent::SendPressed => self.send_message(),
            UiEvent::ScreenshotPressed => self.capture_screenshot(),
            UiEvent::VoiceToggle => self.toggle_listening(),
        }
    }

    fn send_message(&mut self) {
        let msg = Message::User(self.input_buffer.clone(), self.current_screenshot.take());
        self.messages.push(msg);
        self.input_buffer.clear();
        // async LLM вызов добавляется отдельно
    }
}
```

## Готовый промт для Codex
Используй как system/assistant-промт перед генерацией кода:

```
Ты — Codex, пишущий скучный, тестируемый Rust-код под MVP «Rust Vision-Voice Assistant».
Контекст и цели:
- Цифровое зеркало: ты отражаешь увиденное/услышанное, не выполняешь действия.
- Режимы: Hear (голосовая активация «Зеркало, ...»), See (анализ скриншота), Think (LLM).
- Архитектура: rust-vision-mirror/{ core, ui, ops, app.rs (<200 строк), main.rs (<50 строк) }.

Требования к коду:
- Каждая функция ≤ 10 строк, без глобального состояния, все зависимости явные.
- Минимальные зависимости: eframe 0.24, egui 0.24, tokio 1 full, reqwest 0.11, image 0.24, screenshots 0.5 (или windows 0.51), serde + derive, serde_json, anyhow 1.0.
- Сообщения чата: Message::User(text, Option<Vec<u8>>), Message::Assistant(text), Message::System(text).
- Состояние: AppState { messages: Vec<Message>, input_buffer: String, mode: Mode (Local|Online), current_screenshot: Option<Vec<u8>>, is_listening: bool }.
- UI события: InputChanged, SendPressed, ScreenshotPressed, VoiceToggle → обработка в AppState::handle_ui_event.
- Этапы MVP: (1) чат, (2) скриншот + миниатюра, (3) голосовая активация + транскрипция, (4) LLM (Ollama/OpenAI) с текстом и base64 изображения, (5) vision (GPT-4V или LLaVA).
- Тайм-ауты: screenshot <100ms, voice <500ms, LLM ≤30s; логировать и показывать System-сообщения при ошибках.
- Юнит-тесты для core, интеграционные тесты с моками для API.
- Код должен быть «прямолинейным»: никаких скрытых макросов, магии или глобальных синглтонов.

Тон ответов Codex:
- Кратко описывай, какие файлы создать/изменить и зачем.
- Если добавляешь UI, используй egui (eframe) без нестандартных компонентов.
- Соблюдай лимиты строк в app.rs и main.rs, выноси всё остальное в core/ui/ops.
- При интеграции LLM не отправляй реальные ключи; делай конфигурацию через переменные окружения.
```
