use crate::DbPool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Column {
    pub id: i64,
    pub board_id: i64,
    pub name: String,
    pub position: i64,
    pub color: String,
}

pub fn get_columns(pool: &DbPool, board_id: i64) -> Vec<Column> {
    let Ok(db) = pool.get() else { return vec![] };
    let Ok(mut stmt) = db.prepare(
        "SELECT id, board_id, name, position, color FROM columns WHERE board_id = ?1 ORDER BY position ASC"
    ) else { return vec![] };
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

pub fn create_column(pool: &DbPool, board_id: i64, name: &str) -> Option<Column> {
    let Ok(db) = pool.get() else { return None };
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

pub fn rename_column(pool: &DbPool, id: i64, name: &str) -> bool {
    let Ok(db) = pool.get() else { return false };
    db.execute(
        "UPDATE columns SET name = ?1 WHERE id = ?2",
        rusqlite::params![name, id],
    ).is_ok()
}

pub fn update_column_color(pool: &DbPool, id: i64, color: &str) -> bool {
    let Ok(db) = pool.get() else { return false };
    db.execute(
        "UPDATE columns SET color = ?1 WHERE id = ?2",
        rusqlite::params![color, id],
    ).is_ok()
}

pub fn delete_column(pool: &DbPool, id: i64) -> bool {
    let Ok(db) = pool.get() else { return false };
    db.execute("DELETE FROM columns WHERE id = ?1", [id]).is_ok()
}
