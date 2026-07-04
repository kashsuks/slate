use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use crate::db::config::{get_config, set_config};
use super::SharedPool;

#[derive(Deserialize)]
pub struct SetConfigBody {
    pub key: String,
    pub value: String,
}

pub async fn get_config(
    State(pool): State<SharedPool>,
    Path(key): Path<String>,
) -> Result<Json<String>, StatusCode> {
    get_config(&pool, &key)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn set_config(
    State(pool): State<SharedPool>,
    Json(body): Json<SetConfigBody>,
) -> StatusCode {
    if set_config(&pool, &body.key, &body.value) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
