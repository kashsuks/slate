use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use crate::db::cards::{
    Card, get_cards as db_get_cards, create_card as db_create_card,
    update_card as db_update_card, delete_card as db_delete_card,
    move_card as db_move_card, get_board_id_for_card,
};
use crate::db::columns::get_board_id_for_column;
use crate::db::boards::user_can_access_board;
use crate::server::auth::AuthUser;
use super::SharedPool;

#[derive(Deserialize)]
pub struct CreateCardBody {
    pub column_id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateCardBody {
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub due_date: Option<String>,
}

#[derive(Deserialize)]
pub struct MoveCardBody {
    pub column_id: i64,
    pub position: i64,
}

const VALID_PRIORITIES: &[&str] = &["none", "low", "medium", "high"];

fn validate_title(title: &str) -> bool {
    let t = title.trim();
    !t.is_empty() && t.len() <= 255
}

fn validate_description(desc: &Option<String>) -> bool {
    desc.as_ref().is_none_or(|d| d.len() <= 10_000)
}

fn validate_priority(priority: &str) -> bool {
    VALID_PRIORITIES.contains(&priority)
}

fn validate_due_date(due_date: &Option<String>) -> bool {
    due_date.as_ref().is_none_or(|d| {
        let parts: Vec<&str> = d.split('-').collect();
        parts.len() == 3
            && parts[0].len() == 4
            && parts[1].len() == 2
            && parts[2].len() == 2
            && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
    })
}

pub async fn get_cards(
    State(pool): State<SharedPool>,
    Extension(auth): Extension<AuthUser>,
    Path(column_id): Path<i64>,
) -> Result<Json<Vec<Card>>, StatusCode> {
    let board_id = get_board_id_for_column(&pool, column_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    if !user_can_access_board(&pool, board_id, auth.user_id) {
        return Err(StatusCode::FORBIDDEN)
    }
    Ok(Json(db_get_cards(&pool, column_id)))
}

pub async fn create_card(
    State(pool): State<SharedPool>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateCardBody>,
) -> Result<Json<Card>, StatusCode> {
    if !validate_title(&body.title) { return Err(StatusCode::UNPROCESSABLE_ENTITY) }
    let board_id = get_board_id_for_column(&pool, body.column_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    if !user_can_access_board(&pool, board_id, auth.user_id) {
        return Err(StatusCode::FORBIDDEN)
    }
    db_create_card(&pool, body.column_id, &body.title)
        .map(Json)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn update_card(
    State(pool): State<SharedPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateCardBody>,
) -> StatusCode {
    if !validate_title(&body.title) { return StatusCode::UNPROCESSABLE_ENTITY }
    if !validate_description(&body.description) { return StatusCode::UNPROCESSABLE_ENTITY }
    if !validate_priority(&body.priority) { return StatusCode::UNPROCESSABLE_ENTITY }
    if !validate_due_date(&body.due_date) { return StatusCode::UNPROCESSABLE_ENTITY }
    let board_id = match get_board_id_for_card(&pool, id) {
        Some(b) => b,
        None => return StatusCode::NOT_FOUND,
    };
    if !user_can_access_board(&pool, board_id, auth.user_id) { return StatusCode::FORBIDDEN }
    if db_update_card(&pool, id, &body.title, body.description, &body.priority, body.due_date) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn delete_card(
    State(pool): State<SharedPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<i64>,
) -> StatusCode {
    let board_id = match get_board_id_for_card(&pool, id) {
        Some(b) => b,
        None => return StatusCode::NOT_FOUND,
    };
    if !user_can_access_board(&pool, board_id, auth.user_id) { return StatusCode::FORBIDDEN }
    if db_delete_card(&pool, id) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn move_card(
    State(pool): State<SharedPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<i64>,
    Json(body): Json<MoveCardBody>,
) -> StatusCode {
    if body.position < 0 { return StatusCode::UNPROCESSABLE_ENTITY }
    let board_id = match get_board_id_for_card(&pool, id) {
        Some(b) => b,
        None => return StatusCode::NOT_FOUND,
    };
    if !user_can_access_board(&pool, board_id, auth.user_id) { return StatusCode::FORBIDDEN }
    if db_move_card(&pool, id, body.column_id, body.position) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
