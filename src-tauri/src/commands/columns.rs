use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

/// All the fetched datatypes from the DB
#[derive(Serialize, Deserialize)]
pub struct Column {
    pub id: i64,
    pub board_id: i64,
    pub name: String,
    pub position: i64,
    pub color: String,
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
    let Ok(db) = state.db.get() else { return vec![] };
    let mut stmt = db
        .prepare("SELECT id, board_id, name, position, color FROM columns WHERE board_id = ?1 ORDER BY position ASC")
        .unwrap();
    stmt.query_map([board_id], |row| {
        Ok(Column {
            id: row.get(0)?,
            board_id: row.get(1)?,
            name: row.get(2)?,
            position: row.get(3)?,
            color: row.get(4)?,
        })
    })
    .unwrap()
    .filter_map(|r| r.ok())
    .collect()
}

fn validate_name(name: &str) -> bool {
    let trimmed = name.trim();
    !trimmed.is_empty() && trimmed.len() <= 255
}

#[tauri::command]
pub fn create_column(board_id: i64, name: String, state: State<AppState>) -> Option<Column> {
    if !validate_name(&name) { return None }
    let Ok(db) = state.db.get() else { return None };
    let position: i64 = db
        .query_row(
            "SELECT COALESCE(MAX(position) + 1, 0) FROM columns WHERE board_id = ?1",
            [board_id],
            |r| r.get(0),
        )
        .unwrap_or(0);
    db.execute(
        "INSERT INTO columns (board_id, name, position) VALUES (?1, ?2, ?3)",
        rusqlite::params![board_id, name, position],
    ).ok()?;
    let id = db.last_insert_rowid();
    db.query_row(
        "SELECT id, board_id, name, position, color FROM columns WHERE id = ?1",
        [id],
        |row| Ok(Column {
            id: row.get(0)?,
            board_id: row.get(1)?,
            name: row.get(2)?,
            position: row.get(3)?,
            color: row.get(4)?,
        }),
    ).ok()
}

#[tauri::command]
pub fn rename_column(id: i64, name: String, state: State<AppState>) -> bool {
    if !validate_name(&name) { return false }
    let Ok(db) = state.db.get() else { return false };
    db.execute("UPDATE columns SET name = ?1 WHERE id = ?2", [&name, &id.to_string()]).is_ok()
}

#[tauri::command]
pub fn update_column_color(id: i64, color: String, state: State<AppState>) -> bool {
    let Ok(db) = state.db.get() else { return false };
    db.execute(
        "UPDATE columns SET color = ?1 WHERE id = ?2",
        rusqlite::params![color, id],
    ).is_ok()
}

/// BE CAREFUL WHEN USING THIS
///
/// # Arguments
///
/// * `id` - id of the board being deleted
/// * `state` - Current app state
#[tauri::command]
pub fn delete_column(id: i64, state: State<AppState>) -> bool {
    let Ok(db) = state.db.get() else { return false };
    db.execute("DELETE FROM columns WHERE id = ?1", [id]).is_ok()
}
