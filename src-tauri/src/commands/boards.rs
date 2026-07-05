use crate::AppState;
use crate::db::boards::{
    Board, create_board as db_create_board, delete_board as db_delete_board,
    get_boards as db_get_boards, rename_board as db_rename_board,
};
use tauri::State;

fn validate_name(name: &str) -> Result<(), String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("Name cannot be empty".into());
    }
    if trimmed.len() > 255 {
        return Err("Name cannot exceed 255 characters".into());
    }
    Ok(())
}

/// Fetches all board related data from DB
///
/// # Arguments
///
/// * `state` - App state value
///
/// # Examples
///
/// ```
/// [TODO:write some example code]
/// ```
#[tauri::command]
pub fn get_boards(state: State<AppState>) -> Vec<Board> {
    db_get_boards(&state.db)
}

#[tauri::command]
pub fn create_board(name: String, state: State<AppState>) -> Result<Board, String> {
    validate_name(&name)?;
    db_create_board(&state.db, &name)
}

#[tauri::command]
pub fn rename_board(id: i64, name: String, state: State<AppState>) -> bool {
    if validate_name(&name).is_err() { return false }
    db_rename_board(&state.db, id, &name)
}

/// BE EXTREMELY CAREFUL WHEN USING THIS
///
/// # Arguments
///
/// * `id` - board id registered in the DB
/// * `state` - Current app state
///
/// # Examples
///
/// ```
/// [TODO:write some example code]
/// ```
#[tauri::command]
pub fn delete_board(id: i64, state: State<AppState>) -> bool {
    db_delete_board(&state.db, id)
}
