use crate::AppState;
use crate::db::cards::{
    Card, create_card as db_create_card, delete_card as db_delete_card,
    get_cards as db_get_cards, move_card as db_move_card, update_card as db_update_card,
};
use tauri::State;

const VALID_PRIORITIES: &[&str] = &["none", "low", "medium", "high"];

fn validate_title(title: &str) -> bool {
    let t = title.trim();
    !t.is_empty() && t.len() <= 255
}

fn validate_description(desc: &Option<String>) -> bool {
    desc.as_ref().map_or(true, |d| d.len() <= 10_000)
}

fn validate_priority(priority: &str) -> bool {
    VALID_PRIORITIES.contains(&priority)
}

fn validate_due_date(due_date: &Option<String>) -> bool {
    due_date.as_ref().map_or(true, |d| {
        let parts: Vec<&str> = d.split('-').collect();
        parts.len() == 3
            && parts[0].len() == 4
            && parts[1].len() == 2
            && parts[2].len() == 2
            && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
    })
}

#[tauri::command]
pub fn get_cards(column_id: i64, state: State<AppState>) -> Vec<Card> {
    db_get_cards(&state.db, column_id)
}

#[tauri::command]
pub fn create_card(column_id: i64, title: String, state: State<AppState>) -> Option<Card> {
    if !validate_title(&title) { return None }
    db_create_card(&state.db, column_id, &title)
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
    if !validate_title(&title) { return false }
    if !validate_description(&description) { return false }
    if !validate_priority(&priority) { return false }
    if !validate_due_date(&due_date) { return false }
    db_update_card(&state.db, id, &title, description, &priority, due_date)
}

#[tauri::command]
pub fn delete_card(id: i64, state: State<AppState>) -> bool {
    db_delete_card(&state.db, id)
}

#[tauri::command]
pub fn move_card(id: i64, column_id: i64, position: i64, state: State<AppState>) -> bool {
    if position < 0 { return false }
    db_move_card(&state.db, id, column_id, position)
}
