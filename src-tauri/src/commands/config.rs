use crate::AppState;
use crate::db::config::{get_config as db_get_config, set_config as db_set_config};
use tauri::State;

#[tauri::command]
pub fn get_config(key: String, state: State<AppState>) -> Option<String> {
    db_get_config(&state.db, &key)
}

#[tauri::command]
pub fn set_config(key: String, value: String, state: State<AppState>) -> bool {
    db_set_config(&state.db, &key, &value)
}
