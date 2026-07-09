use crate::DbPool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub expires_at: String,
}

pub fn create_session(pool: &DbPool, user_id: i64, token: &str, expires_at: &str) -> Option<Session> {
    let Ok(db) = pool.get() else { return None };
    db.execute(
        "INSERT INTO sessions (user_id, token, expires_at) VALUES (?1, ?2, ?3)",
        rusqlite::params![user_id, token, expires_at],
    ).ok()?;
    let id = db.last_insert_rowid();
    db.query_row(
        "SELECT id, user_id, token, expires_at FROM sessions WHERE id = ?1", 
        [id], 
        |row| Ok(Session {
            id: row.get(0)?,
            user_id: row.get(1)?,
            token: row.get(2)?,
            expires_at: row.get(3)?,
        }),
    ).ok()
}

// validate a token by returning the user_id
// if valid and not expired
pub fn validate_token(pool: &DbPool, token: &str) -> Option<i64> {
    let Ok(db) = pool.get() else { return None };
    db.query_row(
        "SELECT user_id FROM sessions
         WHERE token = ?1
         AND expires_at > datetime('now')", 
        [token], 
        |row| row.get::<_, i64>(0),
    ).ok()
}

pub fn delete_session(pool: &DbPool, token: &str) -> bool {
    let Ok(db) = pool.get() else { return false };
    db.execute(
        "DELETE FROM sessions WHERE token = ?1", 
        [token]
    ).is_ok()
}

// clean up the expired sessions (periodic)
pub fn purge_expired_sessions(pool: &DbPool) {
    let Ok(db) = pool.get() else { return };
    let _ = db.execute(
        "DELETE FROM sessions WHERE expires_at <= datetime('now')",
        [],
    );
}
