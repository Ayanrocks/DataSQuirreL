#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate core;

use database::db::{ConnPool, TableSchema, connect_to_db};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State, http};
// use tauri::menu::{CustomMenuItem, Menu, MenuItem, Submenu};
use storage::{ConnectionStorage, StoredConnection};
use tauri::menu::MenuBuilder;

pub mod config;
pub mod constants;
mod database;
mod storage;

struct ApplicationState {
    dbpool: Mutex<Option<ConnPool>>,
    connection_storage: ConnectionStorage,
}

#[derive(Debug, Serialize, Deserialize)]
struct DBConnectionRequest {
    id: String,
    conn_name: String,
    host_name: String,
    database_name: String,
    database_type: String,
    port: i32,
    user_name: String,
    password: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct TableData<T> {
    columns: Vec<String>,
    rows: Option<Vec<Vec<T>>>,
    row_count: Option<String>,
    table_name: Option<String>,
    query_type: String,
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

#[derive(Debug, Serialize, Deserialize)]
struct TableDataOffsetRequest {
    table_name: String,
    offset: u32,
}

#[tauri::command]
async fn init_connection(
    app: AppHandle,
    req_payload: DBConnectionRequest,
    application_state: State<'_, ApplicationState>,
) -> Result<IPCResponse<String>, ()> {
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

            let stored_conn = StoredConnection {
                id: req_payload.id.clone(),
                conn_name: req_payload.conn_name.clone(),
                host_name: req_payload.host_name,
                database_name: req_payload.database_name,
                database_type: req_payload.database_type.to_string(),
                port: req_payload.port,
                user_name: req_payload.user_name,
            };

            if let Err(e) = application_state.connection_storage.save_connection(
                &app,
                &stored_conn,
                &req_payload.password,
            ) {
                println!("Error {:?}", e);
                return Ok(IPCResponse {
                    status: http::status::StatusCode::OK.as_u16(),
                    error_code: Some(constants::ERR_CODE_STORAGE_FAILED.to_string()),
                    sys_err: Some(e.to_string()),
                    frontend_msg: Some("Failed to store connection details".to_string()),
                    data: None,
                });
            }

            return Ok(IPCResponse {
                status: http::status::StatusCode::OK.as_u16(),
                error_code: None,
                sys_err: None,
                frontend_msg: Some("Database connected successfully".to_string()),
                data: None,
            });
        }
        Err(e) => {
            return Ok(IPCResponse::<_> {
                status: http::status::StatusCode::OK.as_u16(),
                error_code: Some(constants::ERR_CODE_DATABASE_CONN_FAILED.to_string()),
                sys_err: Some(e.to_string()),
                frontend_msg: Some(e.to_string()),
                data: None,
            });
        }
    }
}

#[tauri::command]
fn get_saved_connections(
    app: AppHandle,
    application_state: State<ApplicationState>,
) -> Result<IPCResponse<Vec<StoredConnection>>, ()> {
    match application_state
        .connection_storage
        .get_all_connections(&app)
    {
        Ok(connections) => Ok(IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: None,
            sys_err: None,
            frontend_msg: None,
            data: Some(connections),
        }),
        Err(e) => Ok(IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: Some(constants::ERR_CODE_STORAGE_FAILED.to_string()),
            sys_err: Some(e.to_string()),
            frontend_msg: Some("Failed to retrieve saved connections".to_string()),
            data: None,
        }),
    }
}

#[tauri::command]
fn delete_saved_connection(
    app: AppHandle,
    conn_name: String,
    application_state: State<ApplicationState>,
) -> Result<IPCResponse<String>, ()> {
    match application_state
        .connection_storage
        .delete_connection(&app, &conn_name)
    {
        Ok(_) => Ok(IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: None,
            sys_err: None,
            frontend_msg: Some("Connection deleted successfully".to_string()),
            data: None,
        }),
        Err(e) => Ok(IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: Some(constants::ERR_CODE_STORAGE_FAILED.to_string()),
            sys_err: Some(e.to_string()),
            frontend_msg: Some("Failed to delete connection".to_string()),
            data: None,
        }),
    }
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
                    row_count: Some(t.len().to_string()),
                    rows: Some(vec![t]),
                    table_name: None,
                    query_type: constants::QUERY_TYPE_FETCH_TABLES.to_string(),
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
                };
            }
        }
    })
}

#[tauri::command]
fn fetch_table_data(
    req_payload: TableDataRequest,
    application_state: State<ApplicationState>,
) -> IPCResponse<TableData<String>> {
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

        // fetch table columns
        match table_columns_result {
            Ok(table_columns) => {
                let mut i = 0;
                for t in table_columns {
                    columns.push(t.column_name);
                    i += 1;
                }
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
                };
            }
        };

        // fetch table data
        let table_data_result = application_state
            .dbpool
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .fetch_table_data(&req_payload.table_name)
            .await;

        let mut table_data_rows: Vec<Vec<String>> = vec![vec!["".to_string()]];

        match table_data_result {
            Ok(table_data) => table_data_rows = table_data,
            Err(e) => {
                return IPCResponse::<_> {
                    status: http::status::StatusCode::OK.as_u16(),
                    error_code: Some(
                        constants::ERR_CODE_DATABASE_FETCH_TABLE_DATA_FAILED.to_string(),
                    ),
                    sys_err: Some(e.to_string()),
                    frontend_msg: Some(e.to_string()),
                    data: None,
                };
            }
        };

        // fetch table rows count
        let table_rows_count_result = application_state
            .dbpool
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .fetch_table_rows_count(&req_payload.table_name)
            .await;

        let row_count: String;
        match table_rows_count_result {
            Ok(table_row_count) => row_count = format!("{}", table_row_count.row_count),
            Err(e) => {
                return IPCResponse::<_> {
                    status: http::status::StatusCode::OK.as_u16(),
                    error_code: Some(
                        constants::ERR_CODE_DATABASE_FETCH_TABLE_ROW_COUNT_FAILED.to_string(),
                    ),
                    sys_err: Some(e.to_string()),
                    frontend_msg: Some(e.to_string()),
                    data: None,
                };
            }
        };

        IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: None,
            sys_err: None,
            frontend_msg: None,
            data: Some(TableData {
                columns: columns,
                rows: Some(table_data_rows),
                row_count: Some(row_count),
                table_name: Some(req_payload.table_name),
                query_type: constants::QUERY_TYPE_FETCH_INITIAL_TABLE_DATA.to_string(),
            }),
        }
    })
}

#[tauri::command]
fn fetch_table_data_with_offset(
    req_payload: TableDataOffsetRequest,
    application_state: State<ApplicationState>,
) -> IPCResponse<TableData<String>> {
    tauri::async_runtime::block_on(async {
        let mut columns: Vec<String> = vec![];

        let table_columns_result = application_state
            .dbpool
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .fetch_table_columns(&req_payload.table_name)
            .await;

        // fetch table columns
        match table_columns_result {
            Ok(table_columns) => {
                let mut i = 0;
                for t in table_columns {
                    columns.push(t.column_name);
                    i += 1;
                }
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
                };
            }
        };

        // fetch table rows count
        let table_rows_count_result = application_state
            .dbpool
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .fetch_table_rows_count(&req_payload.table_name)
            .await;

        let row_count: String;
        match table_rows_count_result {
            Ok(table_row_count) => row_count = format!("{}", table_row_count.row_count),
            Err(e) => {
                return IPCResponse::<_> {
                    status: http::status::StatusCode::OK.as_u16(),
                    error_code: Some(
                        constants::ERR_CODE_DATABASE_FETCH_TABLE_ROW_COUNT_FAILED.to_string(),
                    ),
                    sys_err: Some(e.to_string()),
                    frontend_msg: Some(e.to_string()),
                    data: None,
                };
            }
        };

        /*
           Pass the offset and the table name and return the additional data
        */

        let mut table_data_rows: Vec<Vec<String>> = vec![vec!["".to_string()]];

        let table_data_result = application_state
            .dbpool
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .fetch_table_data_with_offset(&req_payload.table_name, &req_payload.offset)
            .await;

        match table_data_result {
            Ok(table_data) => table_data_rows = table_data,
            Err(e) => {
                return IPCResponse::<_> {
                    status: http::status::StatusCode::OK.as_u16(),
                    error_code: Some(
                        constants::ERR_CODE_DATABASE_FETCH_TABLE_DATA_FAILED.to_string(),
                    ),
                    sys_err: Some(e.to_string()),
                    frontend_msg: Some(e.to_string()),
                    data: None,
                };
            }
        };

        IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: None,
            sys_err: None,
            frontend_msg: None,
            data: Some(TableData {
                columns,
                rows: Some(table_data_rows),
                row_count: Some(row_count),
                table_name: Some(req_payload.table_name),
                query_type: constants::QUERY_TYPE_FETCH_OFFSET_TABLE_DATA.to_string(),
            }),
        }
    })
}

fn main() {
    // let quit: MenuItemBuilder = MenuItemBuilder::new("quit".to_string(), "Quit");
    // let close = MenuItemBuilder::new("close".to_string(), "Close");
    // let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    // let menu = Menu::new()
    //     .add_native_item(MenuItem::Copy)
    //     .add_item(MenuItemBuilder::new("hide", "Hide"))
    //     .add_submenu(submenu);
    println!("Starting Tauri App!");

    tauri::Builder::default()
        .setup(|app| {
            /*
            let menu = MenuBuilder::new(app.handle())
                .copy()
                .paste()
                .separator()
                .undo()
                .redo()
                .text("open-url", "Open URL")
                .check("toggle", "Toggle")
                .icon(
                    "show-app",
                    "Show App",
                    app.handle().default_window_icon().cloned().unwrap(),
                )
                .build()
                .expect("Failed to build menu");

            let window = app.get_webview_window("main").unwrap();
            window.set_menu(menu);
            window.set_title("Tauri Database Client");
            */

            // window.open_devtools();
            Ok(())
        })
        // .plugin(tauri_plugin_clipboard_manager::init())
        // .plugin(tauri_plugin_http::init())
        // .plugin(tauri_plugin_process::init())
        // .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        // .plugin(tauri_plugin_shell::init())
        // .plugin(tauri_plugin_dialog::init())
        // .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // .plugin(tauri_plugin_notification::init())
        .manage(ApplicationState {
            dbpool: Default::default(),
            connection_storage: ConnectionStorage::new(),
        })
        .invoke_handler(tauri::generate_handler![
            init_connection,
            fetch_tables,
            fetch_table_data,
            fetch_table_data_with_offset,
            get_saved_connections,
            delete_saved_connection
        ])
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
