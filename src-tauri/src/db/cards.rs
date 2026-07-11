use crate::DbPool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub id: i64,
    pub column_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub due_date: Option<String>,
    pub position: i64,
    pub created_at: String,
}

pub fn get_cards(pool: &DbPool, column_id: i64) -> Vec<Card> {
    let Ok(db) = pool.get() else { return vec![] };
    let Ok(mut stmt) = db.prepare(
        "SELECT id, column_id, title, description, priority, due_date, position, created_at
        FROM cards WHERE column_id = ?1 ORDER BY position ASC"
    ) else { return vec![] };
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

pub fn create_card(pool: &DbPool, column_id: i64, title: &str) -> Option<Card> {
    let Ok(db) = pool.get() else { return None };
    let position: i64 = db
        .query_row(
            "SELECT COALESCE(MAX(position) + 1, 0) FROM cards WHERE column_id = ?1",
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
        }),
    ).ok()
}

pub fn update_card(
    pool: &DbPool,
    id: i64,
    title: &str,
    description: Option<String>,
    priority: &str,
    due_date: Option<String>,
) -> bool {
    let Ok(db) = pool.get() else { return false };
    db.execute(
        "UPDATE cards SET title = ?1, description = ?2, priority = ?3, due_date = ?4 WHERE id = ?5",
        rusqlite::params![title, description, priority, due_date, id],
    ).is_ok()
}

pub fn delete_card(pool: &DbPool, id: i64) -> bool {
    let Ok(db) = pool.get() else { return false };
    db.execute("DELETE FROM cards WHERE id = ?1", [id]).is_ok()
}

pub fn move_card(pool: &DbPool, id: i64, column_id: i64, position: i64) -> bool {
    let Ok(db) = pool.get() else { return false };

    let result = db.query_row(
        "SELECT column_id, position FROM cards WHERE id = ?1",
        [id],
        |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
    );
    let (old_column_id, old_position) = match result {
        Ok(v) => v,
        Err(_) => return false,
    };

    if old_column_id == column_id {
        if old_position < position {
            db.execute(
                "UPDATE cards SET position = position - 1 WHERE column_id = ?1 AND position > ?2 AND position <= ?3 AND id != ?4",
                rusqlite::params![column_id, old_position, position, id],
            ).ok();
        } else {
            db.execute(
                "UPDATE cards SET position = position + 1 WHERE column_id = ?1 AND position >= ?2 AND position < ?3 AND id != ?4",
                rusqlite::params![column_id, position, old_position, id],
            ).ok();
        }
        db.execute(
            "UPDATE cards SET position = ?1 WHERE id = ?2",
            rusqlite::params![position, id],
        ).is_ok()
    } else {
        let tx = match db.unchecked_transaction() {
            Ok(t) => t,
            Err(_) => return false,
        };
        let ok = (|| -> rusqlite::Result<()> {
            tx.execute(
                "UPDATE cards SET position = position - 1 WHERE column_id = ?1 AND position > ?2",
                rusqlite::params![old_column_id, old_position],
            )?;
            tx.execute(
                "UPDATE cards SET position = position + 1 WHERE column_id = ?1 AND position >= ?2 AND id != ?3",
                rusqlite::params![column_id, position, id],
            )?;
            tx.execute(
                "UPDATE cards SET column_id = ?1, position = ?2 WHERE id = ?3",
                rusqlite::params![column_id, position, id],
            )?;
            Ok(())
        })();
        match ok {
            Ok(_) => tx.commit().is_ok(),
            Err(_) => { let _ = tx.rollback(); false }
        }
    }
}

pub fn get_board_id_for_card(pool: &DbPool, card_id: i64) -> Option<i64> {
    let Ok(db) = pool.get() else { return None };
    db.query_row(
        "SELECT c.board_id FROM cards ca
         JOIN columns c ON c.id = ca.column_id
         WHERE ca.id = ?1",
        [card_id],
        |row| row.get(0),
    ).ok()
}

pub fn get_column_id_for_card(pool: &DbPool, card_id: i64) -> Option<i64> {
    let Ok(db) = pool.get() else { return None };
    db.query_row(
        "SELECT column_id FROM cards WHERE id = /1", 
        [card_id], 
        |row| row.get(0),
    ).ok()
}
