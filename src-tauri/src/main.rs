#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::database::db::TableColumns;
use database::db::{connect_to_db, ConnPool, TableSchema};
use serde::{Deserialize, Serialize};
use sqlx::Error;
use std::sync::Mutex;
use tauri::{http, CustomMenuItem, Menu, MenuItem, State, Submenu};

pub mod constants;
mod database;

struct ApplicationState {
    dbpool: Mutex<Option<ConnPool>>,
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
struct TableData<T> {
    columns: Vec<String>,
    rows: Option<Vec<Vec<T>>>,
    table_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct IPCResponse<T> {
    status: u16,
    error_code: Option<String>,
    sys_err: Option<String>,
    frontend_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TableDataRequest {
    table_name: String,
}

#[tauri::command]
fn init_connection(
    req_payload: DBConnectionRequest,
    application_state: State<'_, ApplicationState>,
) -> IPCResponse<String> {
    tauri::async_runtime::block_on(async {
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
                *application_state.dbpool.lock().unwrap() = Some(conn_pool);
                return IPCResponse {
                    status: http::status::StatusCode::OK.as_u16(),
                    error_code: None,
                    sys_err: None,
                    frontend_msg: Some("Database connected successfully".to_string()),
                    data: None,
                };
            }
            Err(e) => {
                return IPCResponse::<_> {
                    status: http::status::StatusCode::OK.as_u16(),
                    error_code: Some(constants::ERR_CODE_DATABASE_CONN_FAILED.to_string()),
                    sys_err: Some(e.to_string()),
                    frontend_msg: Some(e.to_string()),
                    data: None,
                }
            }
        }
    })
}

#[tauri::command]
fn fetch_tables(application_state: State<ApplicationState>) -> IPCResponse<TableData<TableSchema>> {
    tauri::async_runtime::block_on(async {
        let table_result = application_state
            .dbpool
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .fetch_tables()
            .await;

        match table_result {
            Ok(t) => {
                let table_data = TableData {
                    columns: vec![String::from("Table Name")],
                    rows: Some(vec![t]),
                    table_type: "fetch_tables".to_string(),
                };

                return IPCResponse {
                    status: http::status::StatusCode::OK.as_u16(),
                    error_code: None,
                    sys_err: None,
                    frontend_msg: Some("Operation Successful".to_string()),
                    data: Some(table_data),
                };
            }
            Err(e) => {
                return IPCResponse::<_> {
                    status: http::status::StatusCode::OK.as_u16(),
                    error_code: Some(constants::ERR_CODE_DATABASE_FETCH_TABLES_FAILED.to_string()),
                    sys_err: Some(e.to_string()),
                    frontend_msg: Some(e.to_string()),
                    data: None,
                }
            }
        }
    })
}

#[tauri::command]
fn fetch_table_data(
    req_payload: TableDataRequest,
    application_state: State<ApplicationState>,
) -> IPCResponse<TableData<TableSchema>> {
    tauri::async_runtime::block_on(async {
        /*
           3 things needed to display table data
               1. table columns
               2. Total rows
               3. first 600 rows
        */

        let mut columns: Vec<String> = vec![];

        let table_columns_result = application_state
            .dbpool
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .fetch_table_columns(&req_payload.table_name)
            .await;

        match table_columns_result {
            Ok(table_columns) => {
                let mut i = 0;
                for t in table_columns {
                    columns.push(t.column_name);
                    i += 1;
                }

                println!("Columns Data: {:?}", columns)
            }
            Err(e) => {
                return IPCResponse::<_> {
                    status: http::status::StatusCode::OK.as_u16(),
                    error_code: Some(
                        constants::ERR_CODE_DATABASE_FETCH_TABLE_DATA_FAILED.to_string(),
                    ),
                    sys_err: Some(e.to_string()),
                    frontend_msg: Some(e.to_string()),
                    data: None,
                }
            }
        };

        let _ = application_state
            .dbpool
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .fetch_table_data(&req_payload.table_name)
            .await;

        return IPCResponse::<_> {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: Some(constants::ERR_CODE_DATABASE_FETCH_TABLE_DATA_FAILED.to_string()),
            sys_err: Some("".to_string()),
            frontend_msg: Some("".to_string()),
            data: None,
        };
    })
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    tauri::Builder::default()
        .manage(ApplicationState {
            dbpool: Default::default(),
        })
        .menu(menu)
        .invoke_handler(tauri::generate_handler![
            init_connection,
            fetch_tables,
            fetch_table_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
