#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use database::db::connect_to_db;
use serde::{Deserialize, Serialize};
use tauri::http;
// pub mod constants;
mod database;

#[derive(Debug, Serialize, Deserialize)]
struct DBConnectionRequest {
    conn_name: String,
    host_name: String,
    database_name: String,
    port: i32,
    user_name: String,
    password: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct IPCResponse {
    status: u16,
    error_code: Option<String>,
    sys_err: Option<String>,
    frontend_msg: Option<String>,
}

#[tauri::command]
async fn init_connection(req_payload: DBConnectionRequest) -> IPCResponse {
    let dsn = format!(
        "posrgres://{}:{}@{}:{}/{}",
        req_payload.user_name,
        req_payload.password,
        req_payload.host_name,
        req_payload.port,
        req_payload.database_name,
    );

    let _conn = connect_to_db(&dsn, &req_payload.conn_name).await.unwrap();
    return IPCResponse {
        status: http::status::StatusCode::OK.as_u16(),
        error_code: None,
        sys_err: None,
        frontend_msg: Some("Database connected successfully".to_string()),
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![init_connection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
