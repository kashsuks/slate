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
        .invoke_handler(tauri::generate_handler![]) // should call the frontent once written
        .run(tauri::generate_context!())
        .expect("error running app");
}
