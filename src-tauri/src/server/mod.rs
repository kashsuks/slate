pub mod auth;
pub mod boards;
pub mod cards;
pub mod columns;
pub mod config;
pub mod ws;

use crate::DbPool;
use axum::{
    body::Body,
    extract::Request,
    http::{header, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use tower::ServiceExt;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use ws::BoardChannels;

pub type SharedPool = Arc<DbPool>;

// Serves the built SPA shell (index.html) with an honest 200 status.
async fn spa_shell() -> Response {
    match tokio::fs::read_to_string("frontend/index.html").await {
        Ok(html) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
            html,
        )
            .into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "frontend build not found").into_response(),
    }
}

// Serves real static files from frontend/ (JS/CSS/images, with correct
// content types, caching headers etc.), and falls back to the SPA shell
// for any path that isn't an actual file on disk (e.g. /settings,
// /login, /onboarding) so SvelteKit's client-side router can take over.
//
// Note: ServeDir::not_found_service() looks like the obvious tool for
// this, but it hardcodes a 404 status on whatever it's given regardless
// of what the fallback service actually returns - harmless in a real
// browser, but not honest HTTP. Driving ServeDir manually here keeps
// real static files exactly as they were while giving unmatched routes
// a genuine 200.
async fn spa_fallback(req: Request) -> Response {
    match ServeDir::new("frontend").oneshot(req).await {
        Ok(res) if res.status() != StatusCode::NOT_FOUND => res.map(Body::new),
        _ => spa_shell().await,
    }
}

pub async fn run(pool: DbPool, port: u16) {
    let shared = Arc::new(pool);
    let channels: BoardChannels = ws::new_board_channels();

    // public routes that do not require auth
    let public = Router::new()
        .route("/auth/status", get(auth::status))
        .route("/auth/setup", post(auth::setup))
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .with_state(shared.clone());

    let ws_router = Router::new()
        .route("/ws/:board_id", get(ws::ws_handler))
        .with_state((shared.clone(), channels.clone()));

    // Protected routes that require
    // a valid session token
    let protected = Router::new()
        .route("/auth/logout", post(auth::logout))
        .route("/auth/invite", post(auth::generate_invite))
        .route("/boards", get(boards::get_boards))
        .route("/boards", post(boards::create_board))
        .route("/boards/:id/rename", put(boards::rename_board))
        .route("/boards/:id", delete(boards::delete_board))
        .route("/columns", post(columns::create_column))
        .route("/columns/:id", get(columns::get_columns))
        .route("/columns/:id/rename", put(columns::rename_column))
        .route("/columns/:id/color", put(columns::update_column_color))
        .route("/columns/:id", delete(columns::delete_column))
        .route("/cards/:id", get(cards::get_cards))
        .route("/cards", post(cards::create_card))
        .route("/cards/:id", put(cards::update_card))
        .route("/cards/:id", delete(cards::delete_card))
        .route("/cards/:id/move", put(cards::move_card))
        .route("/config/:key", get(config::get_config))
        .route("/config", post(config::set_config))
        .route_layer(middleware::from_fn_with_state(
            shared.clone(),
            auth::require_auth,
        ))
        .with_state(shared.clone())
        .layer(axum::Extension(channels.clone()));

    let api = Router::new()
        .merge(public)
        .merge(ws_router)
        .merge(protected)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let app = Router::new()
        .nest("/api", api)
        .fallback(spa_fallback);

    let addr = format!("0.0.0.0:{}", port);
    println!("Slate server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
