use crate::DbPool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub position: i64,
    pub created_at: String,
}

pub fn get_boards(pool: &DbPool) -> Vec<Board> {
    let Ok(db) = pool.get() else { return vec![] };
    let Ok(mut stmt) = db.prepare(
        "SELECT id, name, position, created_at FROM boards ORDER BY position ASC"
    ) else { return vec![] };
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

pub fn create_board(pool: &DbPool, name: &str) -> Result<Board, String> {
    let db = pool.get().map_err(|e| e.to_string())?;
    let position: i64 = db
        .query_row("SELECT COALESCE(MAX(position) + 1, 0) FROM boards", [], |r| r.get(0))
        .unwrap_or(0);
    db.execute(
        "INSERT INTO boards (name, position) VALUES (?1, ?2)",
        rusqlite::params![name, position],
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

pub fn rename_board(pool: &DbPool, id: i64, name: &str) -> bool {
    let Ok(db) = pool.get() else { return false };
    db.execute(
        "UPDATE boards SET name = ?1 WHERE id = ?2",
        rusqlite::params![name, id],
    ).is_ok()
}

pub fn delete_board(pool: &DbPool, id: i64) -> bool {
    let Ok(db) = pool.get() else { return false };
    db.execute("DELETE FROM boards WHERE id = ?1", [id]).is_ok()
}
