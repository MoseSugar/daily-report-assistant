use crate::{desktop::window_manager::show_capture_window, domain::error::AppError};
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};
use tauri::AppHandle;

#[derive(Clone)]
pub struct HotkeyRuntime {
    manager: Arc<Mutex<Option<GlobalHotKeyManager>>>,
    state: Arc<Mutex<HotkeyState>>,
}

#[derive(Debug, Default)]
struct HotkeyState {
    registered_hotkey: Option<HotKey>,
    last_error: Option<String>,
}

impl Default for HotkeyRuntime {
    fn default() -> Self {
        let manager = GlobalHotKeyManager::new().ok();

        Self {
            manager: Arc::new(Mutex::new(manager)),
            state: Arc::new(Mutex::new(HotkeyState::default())),
        }
    }
}

impl HotkeyRuntime {
    pub fn register_or_replace(
        &self,
        app_handle: &AppHandle,
        accelerator: &str,
    ) -> Result<(), AppError> {
        let hotkey = parse_hotkey(accelerator)?;
        let mut manager_guard = self.manager.lock().expect("hotkey manager mutex poisoned");
        let manager = manager_guard.as_mut().ok_or_else(|| {
            AppError::internal("global hotkey backend is unavailable in the current environment")
        })?;
        let mut state = self.state.lock().expect("hotkey state mutex poisoned");

        if let Some(current_hotkey) = state.registered_hotkey.take() {
            manager
                .unregister(current_hotkey)
                .map_err(|error| AppError::internal(error.to_string()))?;
        }

        manager.register(hotkey).map_err(|error| {
            let message = format!("global hotkey registration failed: {error}");
            state.last_error = Some(message.clone());
            AppError::validation(message)
        })?;

        state.registered_hotkey = Some(hotkey);
        state.last_error = None;
        drop(state);

        ensure_listener_started(app_handle.clone(), self.clone());
        Ok(())
    }

    pub fn last_error(&self) -> Option<String> {
        self.state
            .lock()
            .expect("hotkey state mutex poisoned")
            .last_error
            .clone()
    }
}

fn ensure_listener_started(app_handle: AppHandle, runtime: HotkeyRuntime) {
    static START: std::sync::Once = std::sync::Once::new();

    START.call_once(move || {
        std::thread::spawn(move || {
            let receiver = GlobalHotKeyEvent::receiver();

            while let Ok(event) = receiver.recv() {
                let should_handle = runtime
                    .state
                    .lock()
                    .expect("hotkey state mutex poisoned")
                    .registered_hotkey
                    .as_ref()
                    .map(|hotkey| hotkey.id() == event.id)
                    .unwrap_or(false);

                if should_handle {
                    let _ = show_capture_window(&app_handle);
                }
            }
        });
    });
}

pub fn parse_hotkey(accelerator: &str) -> Result<HotKey, AppError> {
    let normalized_parts = accelerator
        .split('+')
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();

    if normalized_parts.len() < 2 {
        return Err(AppError::validation(
            "global hotkey must include at least one modifier and one key",
        ));
    }

    let mut modifiers = Modifiers::empty();
    let mut key_code = None;

    for part in normalized_parts {
        match part.to_ascii_lowercase().as_str() {
            "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
            "shift" => modifiers |= Modifiers::SHIFT,
            "alt" => modifiers |= Modifiers::ALT,
            "super" | "cmd" | "meta" | "win" => modifiers |= Modifiers::SUPER,
            other => {
                if key_code.is_some() {
                    return Err(AppError::validation(
                        "global hotkey can only contain one primary key",
                    ));
                }

                key_code = Some(parse_key_code(other)?);
            }
        }
    }

    let key_code = key_code
        .ok_or_else(|| AppError::validation("global hotkey must contain one primary key"))?;

    Ok(HotKey::new(Some(modifiers), key_code))
}

fn parse_key_code(value: &str) -> Result<Code, AppError> {
    if value.len() == 1 {
        let character = value.chars().next().expect("single character hotkey");
        if character.is_ascii_alphabetic() {
            return Code::from_str(&format!("Key{}", character.to_ascii_uppercase()))
                .map_err(|_| AppError::validation("unsupported global hotkey key"));
        }

        if character.is_ascii_digit() {
            return Code::from_str(&format!("Digit{}", character))
                .map_err(|_| AppError::validation("unsupported global hotkey key"));
        }
    }

    let upper = value.to_ascii_uppercase();

    if upper.starts_with('F') && upper.len() <= 3 {
        return Code::from_str(&upper)
            .map_err(|_| AppError::validation("unsupported global hotkey key"));
    }

    let mapped = match upper.as_str() {
        "SPACE" => "Space",
        "ENTER" => "Enter",
        "TAB" => "Tab",
        "ESC" | "ESCAPE" => "Escape",
        _ => value,
    };

    Code::from_str(mapped).map_err(|_| AppError::validation("unsupported global hotkey key"))
}

#[cfg(test)]
mod tests {
    use super::parse_hotkey;

    #[test]
    fn parses_default_hotkey() {
        let hotkey = parse_hotkey("Ctrl+Shift+D").expect("default hotkey should parse");
        assert_ne!(hotkey.id(), 0);
    }

    #[test]
    fn rejects_hotkey_without_modifier() {
        let error = parse_hotkey("D").expect_err("single key should be rejected");
        assert!(error.to_string().contains("modifier"));
    }
}
