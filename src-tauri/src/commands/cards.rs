use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub id: i64,
    pub column_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub due_date: Option<String>,
    pub position: i64, // where in the stack of cards is it located 
    pub created_at: String,
}

#[tauri::command]
pub fn get_cards(column_id: i64, state: State<AppState>) -> Vec<Card> {
    let db = state.db.lock().unwrap();
    let mut stmt = db
        .prepare("SELECT id, column_id, title, description, priority, due_date, position, created_at FROM cards WHERE column_id = ?1 ORDER BY position ASC")
        .unwrap();
    stmt.query_map([column_id], |row| {
        Ok(Card {
            id: row.get(0)?,
            column_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            priority: row.get(4)?,
            due_date: row.get(5)?,
            position: row.get(6)?,
            created_at: row.get(7)?,
        })
    })
    .unwrap()
    .filter_map(|r| r.ok())
    .collect()
}

#[tauri::command]
pub fn create_card(
    column_id: i64, 
    title: String, 
    state: State<AppState>
) -> Option<Card> {
    
    let db = state.db.lock().unwrap();
    let position: i64 = db
        .query_row(
            "SELECT COALESCE(MAX(position) +1, 0) FROM cards WHERE column_id = ?1",
            [column_id],
            |r| r.get(0),
        )
        .unwrap_or(0);
    db.execute(
        "INSERT INTO cards (column_id, title, position) VALUES (?1, ?2, ?3)",
        rusqlite::params![column_id, title, position],
    ).ok()?;
    let id = db.last_insert_rowid();
    db.query_row(
        "SELECT id, column_id, title, description, priority, due_date, position, created_at FROM cards WHERE id = ?1",
        [id],
        |row| Ok(Card {
            id: row.get(0)?,
            column_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            priority: row.get(4)?,
            due_date: row.get(5)?,
            position: row.get(6)?,
            created_at: row.get(7)?,
        })
    ).ok()
}

#[tauri::command]
pub fn update_card(
    id: i64,
    title: String,
    description: Option<String>,
    priority: String,
    due_date: Option<String>,
    state: State<AppState>,
) -> bool {
    let db = state.db.lock().unwrap();
    db.execute(
        "UPDATE card SET title = ?1, description = ?2, priority = ?3, due_date = ?4, WHERE id = ?5",
        rusqlite::params![title, description, priority, due_date, id]
    ).is_ok()
}

#[tauri::command]
pub fn delete_card(id: i64, state: State<AppState>) -> bool {
    let db = state.db.lock().unwrap();
    db.execute("DELETE FROM cards WHERE id = ?1", [id]).is_ok()
}

#[tauri::command]
pub fn move_card(
    id: i64, 
    column_id: i64,
    position: i64,
    state: State<AppState>
) -> bool {
    let db = state.db.lock().unwrap();
    // shift the cards to the destination 
    // column to make room
    db.execute(
        "UPDATE cards SET position = position + 1 WHERE column_id = ?1 AND position >= ?2 AND id != ?3",
        rusqlite::params![column_id, position, id],
    ).ok();
    db.execute(
        "UPDATE cards SET column id = ?1, position = ?2, WHERE id = ?3",
        rusqlite::params![column_id, position, id],
    ).is_ok()
}
