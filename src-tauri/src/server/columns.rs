use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use crate::db::columns::{Column, get_columns, create_column, rename_column, update_column_color, delete_column, get_board_id_for_column};
use crate::db::boards::user_can_access_board;
use crate::server::auth::AuthUser;
use super::SharedPool;

#[derive(Deserialize)]
pub struct CreateColumnBody {
    pub board_id: i64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct RenameColumnBody {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateColorBody {
    pub color: String,
}

fn validate_name(name: &str) -> bool {
    let t = name.trim();
    !t.is_empty() && t.len() <= 255
}

pub async fn get_columns(
    State(pool): State<SharedPool>,
    Extension(auth): Extension<AuthUser>,
    Path(board_id): Path<i64>,
) -> Json<Vec<Column>, StatusCode> {
    if !user_can_access_board(&pool, board_id, auth.user_id) {
        return Err(StatusCode::FORBIDDEN)
    }
    Ok(Json(get_columns(&pool, board_id))
}

pub async fn create_column(
    State(pool): State<SharedPool>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateColumnBody>,
) -> Result<Json<Column>, StatusCode> {
    if !validate_name(&body.name) { return Err(StatusCode::UNPROCESSABLE_ENTITY) }
    if !user_can_access_board(&pool, body.board_id, auth.user_id) {
        return Err(StatusCode::FORBIDDEN)
    }
    create_column(&pool, body.board_id, &body.name)
        .map(Json)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn rename_column(
    State(pool): State<SharedPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<i64>,
    Json(body): Json<RenameColumnBody>,
) -> StatusCode {
    if !validate_name(&body.name) { return StatusCode::UNPROCESSABLE_ENTITY }
    let board_id = match get_board_id_for_column(&pool, id) {
        Some(b) => b,
        None => return StatusCode::NOT_FOUND,
    };
    if !user_can_access_board(&pool, board_id, auth.user_id) { return StatusCode::FORBIDDEN }
    if rename_column(&pool, id, &body.name) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn update_column_color(
    State(pool): State<SharedPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateColorBody>,
) -> StatusCode {
    let board_id = match get_board_id_for_column(&pool, id) {
        Some(b) => b,
        None => return StatusCode::NOT_FOUND,
    };
    if !user_can_access_board(&pool, board_id, auth.user_id) { return StatusCode::FORBIDDEN }
    if update_column_color(&pool, id, &body.color) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn delete_column(
    State(pool): State<SharedPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<i64>,
) -> StatusCode {
    let board_id = match get_board_id_for_column(&pool, id) {
        Some(b) => b,
        None => return StatusCode::NOT_FOUND,
    };
    if !user_can_access_board(&pool, board_id, auth.user_id) { return StatusCode::FORBIDDEN }
    if delete_column(&pool, id) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
