use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use crate::{commands::boards::get_boards, db::boards::{Board, create_board, delete_board, get_boards, rename_board}};
use super::SharedPool;

#[derive(Deserialize)]
pub struct CreateBoardBody {
    pub name: String,
}

#[derive(Deserialize)]
pub struct RenameBoardBody {
    pub name: String,
}

fn validate_name(name: &str) -> bool {
    let t = name.trim();
    !t.is_empty() && t.len() <= 255
}

pub async fn get_boards(
    State(pool): State<SharedPool>,
) -> Json<Vec<Board>> {
    Json(get_boards(&pool))
}

pub async fn create_board(
    State(pool): State<SharedPool>,
    Json(body): Json<CreateBoardBody>,
) -> Result<Json<Board>, StatusCode> {
    if !validate_name(&body.name) { return Err(StatusCode::UNPROCESSABLE_ENTITY) }
    create_board(&pool, &body.name)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn rename_board(
    State(pool): State<SharedPool>,
    Path(id): Path<i64>,
    Json(body): Json<RenameBoardBody>,
) -> StatusCode {
    if !validate_name(&body.name) { return StatusCode::UNPROCESSABLE_ENTITY }
    if rename_board(&pool, id, &body.name) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn delete_board(
    State(pool): State<SharedPool>,
    Path(id): Path<i64>,
) -> StatusCode {
    if delete_board(&pool, id) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
