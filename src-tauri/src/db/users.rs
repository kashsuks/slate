use crate::DbPool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub created_at: String,
}

// stored seperately - never serialized out
// of the client
pub struct UserRecord {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

pub fn get_user_by_id(pool: &DbPool, id: i64) -> Option<User> {
    let Ok(db) = pool.get() else { return None };
    db.query_row(
        "SELECT id, username, created_at FROM users WHERE id = ?1",
        [id],
        |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                created_at: row.get(2)?,
            })
        },
    )
    .ok()
}

pub fn get_user_by_username(pool: &DbPool, username: &str) -> Option<UserRecord> {
    let Ok(db) = pool.get() else { return None };
    db.query_row(
        "SELECT id, username, password_hash FROM users WHERE username = ?1",
        [username],
        |row| {
            Ok(UserRecord {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
            })
        },
    )
    .ok()
}

pub fn create_user(pool: &DbPool, username: &str, password_hash: &str) -> Option<User> {
    let Ok(db) = pool.get() else { return None };
    db.execute(
        "INSERT INTO users (username, password_hash) VALUES (?1, ?2)",
        rusqlite::params![username, password_hash],
    )
    .ok()?;
    let id = db.last_insert_rowid();
    get_user_by_id(pool, id)
}

pub fn username_exists(pool: &DbPool, username: &str) -> bool {
    let Ok(db) = pool.get() else { return false };
    db.query_row(
        "SELECT COUNT(*) FROM users WHERE username = ?1",
        [username],
        |row| row.get::<_, i64>(0),
    )
    .unwrap_or(0)
        > 0
}

// check whether the server has any users at all
// used to determine if the server needs
// initial setup
pub fn has_any_users(pool: &DbPool) -> bool {
    let Ok(db) = pool.get() else { return false };
    db.query_row("SELECT COUNT(*) FROM users", [], |row| row.get::<_, i64>(0))
        .unwrap_or(0)
        > 0
}
