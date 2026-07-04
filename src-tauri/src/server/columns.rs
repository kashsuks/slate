use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use crate::{commands::columns::get_columns, db::columns::{Column, create_column, delete_column, get_columns, rename_column, update_column_color}};
use super::SharedPool;

#[derive(Deserialize)]
pub struct CreateColumnBody {
    pub board_id: i64,
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
    Path(board_id): Path<i64>,
) -> Json<Vec<Column>> {
    Json(get_columns(&pool, board_id)
}

pub async fn create_column(
    State(pool): State<SharedPool>,
    Json(body): Json<CreateColumnBody>,
) -> Result<Json<Column>, StatusCode> {
    if !validate_name(&body.name) { return Err(StatusCode::UNPROCESSABLE_ENTITY) }
    create_column(&pool, body.board_id, &body.name)
        .map(Json)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn rename_column(
    State(pool): State<SharedPool>,
    Path(id): Path<i64>,
    Json(body): Json<RenameColumnBody>,
) -> StatusCode {
    if !validate_name(&body.name) { return StatusCode::UNPROCESSABLE_ENTITY }
    if rename_column(&pool, id, &body.name) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn update_column_color(
    State(pool): State<SharedPool>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateColorBody>,
) -> StatusCode {
    if update_column_color(&pool, id, &body.color) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn delete_column(
    State(pool): State<SharedPool>,
    Path(id): Path<i64>,
) -> StatusCode {
    if delete_column(&pool, id) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
