use tauri_app_lib::{AppState, DbPool, init_db};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tauri::Manager;
use std::path::PathBuf;

fn build_pool(db_path: PathBuf) -> DbPool {
    let manager = SqliteConnectionManager::file(db_path);
    Pool::builder()
        .max_size(8)
        .build(manager)
        .expect("Failed to create DB pool")
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let server_mode = args.contains(&"--server".to_string());

    if server_mode {
        // server mode - headless with no tauri
        let db_path = std::env::var("SLATE_DB_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("slate.db"));
        let pool = build_pool(db_path);
        init_db(&pool);
        let port: u16 = std::env::var("SLATE_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);
        tauri_app_lib::server::run(pool, port).await;
    } else {
        // desktop mode - normal tauri app
        tauri::Builder::default()
            .setup(|app| {
                let data_dir = app.path().app_data_dir().unwrap();
                std::fs::create_dir_all(&data_dir).unwrap();
                let db_path = data_dir.join("slate.db");
                let pool = build_pool(db_path);
                init_db(&pool);
                app.manage(AppState { db: pool });
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                tauri_app_lib::commands::config::get_config,
                tauri_app_lib::commands::config::set_config,
                tauri_app_lib::commands::boards::get_boards,
                tauri_app_lib::commands::boards::create_board,
                tauri_app_lib::commands::boards::rename_board,
                tauri_app_lib::commands::boards::delete_board,
                tauri_app_lib::commands::columns::get_columns,
                tauri_app_lib::commands::columns::create_column,
                tauri_app_lib::commands::columns::rename_column,
                tauri_app_lib::commands::columns::update_column_color,
                tauri_app_lib::commands::columns::delete_column,
                tauri_app_lib::commands::cards::get_cards,
                tauri_app_lib::commands::cards::create_card,
                tauri_app_lib::commands::cards::update_card,
                tauri_app_lib::commands::cards::delete_card,
                tauri_app_lib::commands::cards::move_card,
            ])
            .run(tauri::generate_context!())
            .expect("error running app");
    }
}
