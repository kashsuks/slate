pub mod boards;
pub mod columns;
pub mod cards;
pub mod config;

use axum::{Router, routing::{get, post, put, delete}};
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::ServeDir;
use crate::DbPool;
use std::sync::Arc;

pub type SharedPool = Arc<DbPool>;

pub async fn run(pool: DbPool, port: u16) {
    let shared = Arc::new(pool);

    let api = Router::new()
        //boards
        .route("/boards", get(boards::get_boards))
        .route("/boards", post(boards::create_board))
        .route("/boards/:id/rename", put(boards::rename_board))
        .route("/boards/:id", delete(boards::delete_board))
        //columns
        .route("/columns", post(columns::create_column))
        .route("/columns/:board_id", get(column::get_column))
        .route("/columns/:id/rename", put(columns::rename_column))
        .route("/columns/:id/color", put(columns::update_column_color))
        .route("/columns/:id", delete(columns::delete_column))
        //cards
        .route("/cards/:column_id", get(cards::get_cards))
        .route("/cards", post(cards::create_card))
        .route("/cards/:id", put(cards::update_card))
        .route("/cards/:id", delete(cards::delete_card))
        .route("/cards/:id/move", put(cards::move_card))
        // config
        .route("/config/:key", get(config::get_config))
        .route("/config", post(config::set_config))
        .with_state(shared)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        );

    let app = Router::new()
        .nest("/api", api)
        // Serve the built SvelteKit frontend from ./frontend
        // (we'll populate this folder in the Dockerfile)
        .fallback_service(ServeDir::new("frontend"));

    let addr = format!("0.0.0.0:{}", port);
    println!("Slate server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
