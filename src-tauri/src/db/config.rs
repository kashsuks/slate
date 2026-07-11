use crate::DbPool;

pub fn get_config(pool: &DbPool, key: &str) -> Option<String> {
    let Ok(db) = pool.get() else { return None };
    db.query_row(
        "SELECT value FROM app_config WHERE key = ?1",
        [key],
        |row| row.get(0),
    )
    .ok()
}

pub fn set_config(pool: &DbPool, key: &str, value: &str) -> bool {
    let Ok(db) = pool.get() else { return false };
    db.execute(
        "INSERT INTO app_config (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        rusqlite::params![key, value],
    )
    .is_ok()
}
