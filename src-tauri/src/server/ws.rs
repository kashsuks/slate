use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    response::Response,
};
use dashmap::Dashmap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

// one broadcast channel per board_id
// senders are cloned for each new subscriber
pub type BoardChannels = Arc<DashMap<i64, broadcast::Sender<String>>>;

pub fn new_board_channels() -> BoardChannels {
    Arc::new(DashMap::new())
}

// get or create a channel for a board
fn board_sender(channels: &BoardChannels, board_id: i64) -> broadcast::Sender<String> {
    channels
        .entry(board_id)
        .or_insert_with(|| broadcast::channel::<String>(64).0)
        .clone()
}

// broadcast an event to all subscribers of a board (except the sender)
pub fn broadcast_to_board(channels: &BoardChannels, board_id: i64, event: WsEvent) {
    if let Ok(json) = serde_json::to_string(&event) {
        if let Some(tx) = channels.get(&board_id) {
            // ignore any of the send errors since
            // no subscribers in channel is fine
            let _ = tx.send(json);
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsEvent {
    CardCreated { board_id: i64, card: serde_json::Value },
    CardUpdated { board_id: i64, card: serde_json::Value },
    CardDeleted { board_id: i64, card_id: i64, column_id: i64 },
    CardMoved { board_id: i64, card_id: i64, from_column_id: i64, to_column_id: i64, position: i64 },
    ColumnCreated { board_id: i64, column: serde_json::Value },
    ColumnRenamed { board_id: i64, column_id: i64, name: String },
    ColumnColor { board_id: i64, column_id: i64, color: String },
    ColumnDeleted { board_id: i64, column_id: i64 },
    BoardRenamed { board_id: i64, name: String },
}

// WebSocket handler and helpers

#[derive(Deserialize)]
pub struct WsQuery {
    pub token: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State((pool, channels)): State<(super::SharedPool, BoardChannels)>,
    Path(board_id): Path<i64>,
    Query(query): Query<WsQuery>,
) -> Response {
    // validate a token before upgrading it
    let user_id = crate::db::sessions::validate_token(&pool, &query.token);
    if user_id.is_none() {
        // reject - return a plain 401 response
        return axum::response::IntoResponse::into_response((
            axum::http::StatusCode::UNAUTHORIZED,
            "Invalid or expired token",
        ));
    }

    let user_id = user_id.unwrap();

    // check for board access
    if !crate::db::boards::user_can_access_board(&pool, board_id, user_id) {
        return axum::response::IntoResponse::into_response((
            axum::http::StatusCode::FORBIDDEN,
            "Access denied",
        ));
    }

    let tx = board_sender(&channels, board_id);
    ws.on_upgrade(move |socket| handle_socket(socket, tx))
    // above should allow us to upgrade the
    // token that is being used for subs
}

async fn handle_socket(mut socket: WebSocket, tx: broadcast::Sender<String>) {
    let mut rx = tx.subscribe(); // actually receive the signals for sender through rx
    let id = Uuid::new_v4();

    loop {
        tokio::select! {
            // forward the broadcast events to this client
            Ok(msg) = rx.recv() => {
                if socket.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
            // handle incoming messages from the client (ping/close)
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_)) | None => break,
                    Some(Ok(Message::Ping(data))) => {
                        let _ = socket.send(Message::Pong(data)).await;
                    }
                    _ => {}
                }
            }
        }
    }

    drop(id); // connection closed
}
