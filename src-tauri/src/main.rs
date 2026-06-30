use tauri_app_lib::{AppState, init_db};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app.path().app_data_dir().unwrap();
            std::fs::create_dir_all(&data_dir).unwrap();
            let db_path = data_dir.join("slate.db");
            let conn = Connection::open(db_path).unwrap();
            init_db(&conn); // might require a time stop if the connection fails
                            // or exceeds a time limit
            app.manage(AppState { db: Mutex::new(conn) });
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
        ]) // should call the frontent once written
        .run(tauri::generate_context!())
        .expect("error running app");
}
