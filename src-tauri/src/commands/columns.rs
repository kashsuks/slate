use crate::db::columns::{
    create_column as db_create_column, delete_column as db_delete_column,
    get_columns as db_get_columns, rename_column as db_rename_column,
    update_column_color as db_update_column_color, Column,
};
use crate::AppState;
use tauri::State;

fn validate_name(name: &str) -> bool {
    let trimmed = name.trim();
    !trimmed.is_empty() && trimmed.len() <= 255
}

/// Fetch the required information from the DB
///
/// # Arguments
///
/// * `board_id` - DB id of the specified board
/// * `state` - Current app state
///
/// # Examples
///
/// ```
/// [TODO:write some example code]
/// ```
#[tauri::command]
pub fn get_columns(board_id: i64, state: State<AppState>) -> Vec<Column> {
    db_get_columns(&state.db, board_id)
}

#[tauri::command]
pub fn create_column(board_id: i64, name: String, state: State<AppState>) -> Option<Column> {
    if !validate_name(&name) {
        return None;
    }
    db_create_column(&state.db, board_id, &name)
}

#[tauri::command]
pub fn rename_column(id: i64, name: String, state: State<AppState>) -> bool {
    if !validate_name(&name) {
        return false;
    }
    db_rename_column(&state.db, id, &name)
}

#[tauri::command]
pub fn update_column_color(id: i64, color: String, state: State<AppState>) -> bool {
    db_update_column_color(&state.db, id, &color)
}

/// BE CAREFUL WHEN USING THIS
///
/// # Arguments
///
/// * `id` - id of the board being deleted
/// * `state` - Current app state
#[tauri::command]
pub fn delete_column(id: i64, state: State<AppState>) -> bool {
    db_delete_column(&state.db, id)
}
