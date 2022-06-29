#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use database::db::{connect_to_db, ConnPool};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{http, State};

pub mod constants;
mod database;

struct ApplicationState {
    DBPool: Mutex<Option<ConnPool>>,
}

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
struct TableData {
    columns: Vec<String>,
    rows: Option<Vec<String>>,
    table_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct IPCResponse {
    status: u16,
    error_code: Option<String>,
    sys_err: Option<String>,
    frontend_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<TableData>>,
}

#[tauri::command]
async fn init_connection(
    req_payload: DBConnectionRequest,
    application_state: State<'_, ApplicationState>,
) -> Result<IPCResponse, ()> {
    let conn_result = connect_to_db(
        &req_payload.user_name,
        &req_payload.password,
        &req_payload.host_name,
        &req_payload.port,
        &req_payload.database_name,
        &req_payload.conn_name,
    )
    .await;

    match conn_result {
        Ok(conn_pool) => {
            *application_state.DBPool.lock().unwrap() = Some(conn_pool);
            return Ok(IPCResponse {
                status: http::status::StatusCode::OK.as_u16(),
                error_code: None,
                sys_err: None,
                frontend_msg: Some("Database connected successfully".to_string()),
                data: None,
            });
        }
        Err(e) => {
            return Ok(IPCResponse {
                status: http::status::StatusCode::OK.as_u16(),
                error_code: Some(constants::ErrCodeDatabaseConnFailed.to_string()),
                sys_err: Some(e.to_string()),
                frontend_msg: Some(e.to_string()),
                data: None,
            })
        }
    }
}

#[tauri::command]
async fn fetch_tables(application_state: State<'_, ApplicationState>) -> Result<IPCResponse, ()> {
    // let db_conn = *application_state.DBPool.lock().unwrap();

    // match db_conn {
    //     Some(db) => {}
    //     None => todo!(),
    // };

    return Ok(IPCResponse {
        status: http::status::StatusCode::OK.as_u16(),
        error_code: Some(constants::ErrCodeDatabaseConnFailed.to_string()),
        sys_err: None,
        frontend_msg: Some("".to_string()),
        data: None,
    });
}

fn main() {
    tauri::Builder::default()
        .manage(ApplicationState {
            DBPool: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![init_connection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
