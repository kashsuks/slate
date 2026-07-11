use super::SharedPool;
use crate::db::boards::{
    create_board as db_create_board, delete_board as db_delete_board, get_boards as db_get_boards,
    rename_board as db_rename_board, user_can_access_board, user_owns_board, Board,
};
use crate::server::auth::AuthUser;
use crate::server::ws::{broadcast_to_board, BoardChannels};
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

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
    Extension(auth): Extension<AuthUser>,
) -> Json<Vec<Board>> {
    Json(db_get_boards(&pool, Some(auth.user_id)))
}

pub async fn create_board(
    State(pool): State<SharedPool>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateBoardBody>,
) -> Result<Json<Board>, StatusCode> {
    if !validate_name(&body.name) {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    db_create_board(&pool, &body.name, Some(auth.user_id))
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn rename_board(
    State(pool): State<SharedPool>,
    Extension(auth): Extension<AuthUser>,
    Extension(channels): Extension<BoardChannels>,
    Path(id): Path<i64>,
    Json(body): Json<RenameBoardBody>,
) -> StatusCode {
    if !validate_name(&body.name) {
        return StatusCode::UNPROCESSABLE_ENTITY;
    }
    if !user_can_access_board(&pool, id, auth.user_id) {
        return StatusCode::FORBIDDEN;
    }
    if !db_rename_board(&pool, id, &body.name) {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    broadcast_to_board(
        &channels,
        id,
        super::ws::WsEvent::BoardRenamed {
            board_id: id,
            name: body.name.clone(),
        },
    );
    StatusCode::NO_CONTENT
}

pub async fn delete_board(
    State(pool): State<SharedPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<i64>,
) -> StatusCode {
    // allow only the owners to delete
    if !user_owns_board(&pool, id, auth.user_id) {
        return StatusCode::FORBIDDEN;
    }
    if db_delete_board(&pool, id) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
