use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub position: i64,
    pub created_at: String,
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
    let db = state.db.get().unwrap();
    let mut stmt = db
        .prepare("SELECT id, name, position, created_at FROM boards ORDER BY position ASC")
        .unwrap();
    stmt.query_map([], |row| {
        Ok(Board {
            id: row.get(0)?,
            name: row.get(1)?,
            position: row.get(2)?,
            created_at: row.get(3)?,
        })
    })
    .unwrap()
    .filter_map(|r| r.ok())
    .collect()
}

fn validate_name(name: &str) -> Result<(), String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("Name cannot exceed 255 characters".into());
    }
    Ok(())
}

#[tauri::command]
pub fn create_board(name: String, state: State<AppState>) -> Result<Board, String> {
    validate_name(&name)?;
    let db = state.db.get().map_err(|e| e.to_string())?;
    let position: i64 = db
        .query_row("SELECT COALESCE(MAX(position) + 1, 0) FROM boards", [], |r| r.get(0))
        .unwrap_or(0);
    db.execute(
        "INSERT INTO boards (name, position) VALUES (?1, ?2)",
        rusqlite::params![&name, position],
    ).map_err(|e| e.to_string())?;
    let id = db.last_insert_rowid();
    db.query_row(
        "SELECT id, name, position, created_at FROM boards WHERE id = ?1",
        [id],
        |row| Ok(Board {
            id: row.get(0)?,
            name: row.get(1)?,
            position: row.get(2)?,
            created_at: row.get(3)?,
        }),
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn rename_board(id: i64, name: String, state: State<AppState>) -> bool {
    if validate_name(&name).is_err() { return false }
    let Ok(db) = state.db.get() else { return false };
    db.execute("UPDATE boards SET name = ?1 WHERE id = ?2", [&name, &id.to_string()]).is_ok()
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
    let Ok(db) = state.db.get() else { return false };
    db.execute("DELETE FROM boards WHERE id = ?1", [id]).is_ok()
}
