//! WordCraft's settings, where its desktop binary keeps them on Linux
//! (`upstream/apps/wordcraft/src/main.rs`): `ui.json`, `custom-dictionary.txt`, `fonts/` and
//! `recovery/` in the settings folder, which is `$XDG_CONFIG_HOME/wordcraft`. On Android,
//! craft-android points `XDG_CONFIG_HOME` at the app's private storage.

use std::path::PathBuf;

use wordcraft_ui_egui::{UiState, WordApp};

/// Runs without preferences (`WORDCRAFT_NO_PREFS`, as for the desktop binary) neither read nor
/// write them.
fn enabled() -> bool {
    std::env::var_os("WORDCRAFT_NO_PREFS").is_none()
}

/// WordCraft's settings folder, unless preferences are off.
pub fn settings_dir() -> Option<PathBuf> {
    wordcraft_control_key::settings_dir().filter(|_| enabled())
}

fn ui_json() -> Option<PathBuf> {
    settings_dir().map(|d| d.join("ui.json"))
}

/// Restore the saved interface state (recent documents, options, theme, language).
pub fn load(app: &mut WordApp) {
    if let Some(p) = ui_json()
        && let Ok(bytes) = std::fs::read(&p)
        && let Some(ui) = UiState::from_json(&bytes)
    {
        app.apply_prefs(ui);
    }
}

/// The words Add to Dictionary kept.
pub fn load_user_dictionary(app: &mut WordApp) {
    if let Some(p) = settings_dir().map(|d| d.join(wordcraft_engine::Session::USER_DICT_FILE)) {
        wordcraft_engine::proof::load_user_dictionary(&p);
        app.session.user_dict_path = Some(p);
    }
}

/// Fonts the user added (File › Options › Fonts) live in `fonts` in the settings folder.
pub fn load_user_fonts() {
    let Some(dir) = settings_dir().map(|d| d.join("fonts")) else { return };
    let (loaded, skipped) = wordcraft_fonts::FontDb::global().load_user_fonts(&dir);
    if !loaded.is_empty() || !skipped.is_empty() {
        log::info!("dexwordcraft: user fonts: {} loaded, {} skipped from {}", loaded.len(), skipped.len(), dir.display());
    }
}

/// Write the preferences, logging why when that fails.
pub fn save(app: &WordApp) {
    if let Err(e) = try_save(app) {
        log::warn!("dexwordcraft: preferences: {e}");
    }
}

/// WordCraft's `Services::save_prefs`: `Ok(false)` when nothing was written because preferences
/// are off or there is no settings folder.
pub fn try_save(app: &WordApp) -> Result<bool, String> {
    let Some(path) = ui_json() else { return Ok(false) };
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(&app.prefs()).map_err(|e| format!("cannot serialize: {e}"))?;
    write_atomic(&path, &bytes).map_err(|e| format!("cannot save {}: {e}", path.display()))?;
    Ok(true)
}

/// Write `bytes` to a sibling temporary file and rename it over `path`, so an interrupted write
/// (Android ending the process) leaves the previous preferences in place.
fn write_atomic(path: &std::path::Path, bytes: &[u8]) -> std::io::Result<()> {
    let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| "ui.json".into());
    let tmp = path.with_file_name(format!(".{name}.{}.tmp", std::process::id()));
    let written = std::fs::write(&tmp, bytes).and_then(|()| std::fs::rename(&tmp, path));
    if written.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    written
}

#[cfg(test)]
mod tests {
    use super::write_atomic;

    #[test]
    fn a_failed_write_keeps_the_previous_preferences() {
        let dir = std::env::temp_dir().join(format!("dexwordcraft-prefs-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("ui.json");
        write_atomic(&path, b"old").unwrap();
        write_atomic(&path, b"new").unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"new");
        std::fs::create_dir(dir.join(format!(".ui.json.{}.tmp", std::process::id()))).unwrap();
        assert!(write_atomic(&path, b"newer").is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"new");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
