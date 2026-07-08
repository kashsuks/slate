use crate::DbPool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub position: i64,
    pub created_at: String,
}

pub fn get_boards(pool: &DbPool, user_id: Option<i64>) -> Vec<Board> {
    let Ok(db) = pool.get() else { return vec![] };
    let query = match user_id {
        Some(_) => "
            SELECT DISTINCT b.id, b.name, b.owner_id, b.position, b.created_at
            FROM boards b
            LEFT JOIN board_members bm ON bm.board_id = id
            WHERE b.owner_id = ?1 OR bm.user_id = ?1
            ORDER BY b.position ASC",
        None => "
            SELECT id, name, owner_id, position, created_at
            FROM boards ORDER BY position ASC",
    };
    let Ok(mut stmt) = db.prepare(query) else { return vec![] };
    let rows = match user_id {
        Some(id) => stmt.query_map(rusqlite::params![id], map_board),
        None => stmt.query_map([], map_board),
    };
    rows.unwrap().filter_map(|r| r.ok()).collect()
}

fn map_board(row: &rusqlite::Row) -> rusqlite::Result<Board> {
    Ok(Board {
        id: row.get(0)?,
        name: row.get(1)?,
        owner_id: row.get(2)?,
        position: row.get(3)?,
        created_at: row.get(4)?,
    })
}

pub fn create_board(pool: &DbPool, name: &str, owner_id: Option<i64>) -> Result<Board, String> {
    let db = pool.get().map_err(|e| e.to_string())?;
    let position: i64 = db
        .query_row("SELECT COALESCE(MAX(position) + 1, 0) FROM boards", [], |r| r.get(0))
        .unwrap_or(0);
    db.execute(
        "INSERT INTO boards (name, owner_id, position) VALUES (?1, ?2, ?3)", 
        rusqlite::params![name, owner_id, position],
    ).map_err(|e| e.to_string())?;
    let id = db.last_insert_rowid();

    // add owner to board_members so that permission
    // checks are uniform
    if let Some(uid) = owner_id {
        let _ = db.execute(
            "INSERT INTO board_members (board_id, user_id, role) VALUES (?1, ?2, 'owner')",
            rusqlite::params![id, uid],
        );
    }

    db.query_row(
        "SELECT id, name, owner_id, position, created_at, FROM boards WHERE id = ?1", 
        [id], 
        |row| Ok(Board {
            id: row.get(0)?,
            name: row.get(1)?,
            owner_id: row.get(2)?,
            position: row.get(3)?,
            created_at: row.get(4)?,
        })
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

pub fn user_can_access_board(pool: &DbPool, board_id: i64, user_id: i64) -> bool {
    let Ok(db) = pool.get() else { return false };
    db.query_row(
        "SELECT COUNT(*) FROM board_members
         WHERE board_id = ?1 AND user_id = ?2", 
        rusqlite::params![board_id, user_id], 
        |row| row.get::<_, i64>(0),
    ).unwrap_or(0) > 0
}

// Check if a user owns a board ( needed for delete )
pub fn user_owns_board(pool: &DbPool, board_id: i64, user_id: i64) -> bool {
    let Ok(db) = pool.get() else { return false };
    db.query_row(
        "SELECT COUNT(*) FROM boards
         WHERE id = ?1 AND owner_id = ?2", 
        rusqlite::params![board_id, user_id], 
        |row| row.get::<_, i64>(0),
    ).unwrap_or(0) > 0
}
