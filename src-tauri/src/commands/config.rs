use crate::AppState;
use tauri::State;

#[tauri::command]
pub fn get_config(key: String, state: State<AppState>) -> Option<String> {
    let db = state.db.lock().unwrap();
    db.query_row(
        "SELECT value FROM app_config WHERE key = ?1",
        [&key],
        |row| row.get(0),
    ).ok()
}

#[tauri::command]
pub fn set_config(key: String, value: String, state: State<AppState>) -> bool {
    let db = state.db.lock().unwrap();
    db.execute(
        "INSERT INTO app_config (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        [&key, &value],
    ).is_ok()
}
