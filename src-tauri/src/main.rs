#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate core;

use database::db::connect_to_db;
use std::collections::HashMap;
use std::sync::Mutex;
use storage::{ConnectionStorage, StoredConnection};
use tauri::{AppHandle, Manager, State, TitleBarStyle, Url, http};
use tauri::{WebviewUrl, WebviewWindowBuilder};

use crate::constants::APP_NAME;
use crate::types::api_objects::{
    ApplicationState, DBConnectionRequest, DashboardData, DashboardDataRequest, IPCResponse,
    SchemaData, TableData, TableDataOffsetRequest, TableDataRequest,
};
mod logging;

pub mod config;
pub mod constants;
mod database;
mod storage;
mod types;
pub mod sql_console_storage;

#[tauri::command]
async fn init_connection(
    app: AppHandle,
    req_payload: DBConnectionRequest,
    application_state: State<'_, ApplicationState>,
) -> Result<IPCResponse<String>, ()> {
    log_function!(init_connection);
    log_info!(
        "Attempting to connect to database: {}",
        req_payload.database_name
    );

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
            log_info!(
                "Successfully connected to database: {}",
                req_payload.database_name
            );
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

            if let Err(e) = application_state
                .connection_storage
                .save_connection(&stored_conn, &req_payload.password)
            {
                log_error!("Failed to store connection details: {}", e);
                return Ok(IPCResponse {
                    status: http::status::StatusCode::OK.as_u16(),
                    error_code: Some(constants::ERR_CODE_STORAGE_FAILED.to_string()),
                    sys_err: Some(e.to_string()),
                    frontend_msg: Some("Failed to store connection details".to_string()),
                    data: None,
                });
            }

            log_info!("Connection details stored successfully");

            let webview_url =
                WebviewUrl::External(Url::parse("http://localhost:3001/dashboard").unwrap());
            let window_label = format!("connection-window-{}", req_payload.conn_name);

            // Add connection to active connection map
            let mut active_connection_map = application_state.active_connection_map.lock().unwrap();
            active_connection_map.insert(window_label.clone(), stored_conn.clone());

            // Check if window already exists for this connection
            if app.get_webview_window(&window_label).is_some() {
                // Window already exists, just focus it
                if let Some(window) = app.get_webview_window(&window_label) {
                    window.set_focus().unwrap();
                }
            } else {
                // Create new window for this connection
                let window = WebviewWindowBuilder::new(&app, &window_label, webview_url)
                    .title(format!("{} - {}", req_payload.conn_name.clone(), APP_NAME))
                    .inner_size(1450.0, 950.0)
                    .center()
                    .title_bar_style(TitleBarStyle::Overlay)
                    .resizable(true)
                    .decorations(true)
                    .visible(true)
                    .fullscreen(false)
                    .build();

                match window {
                    Ok(w) => {
                        w.start_dragging().unwrap();
                    }
                    Err(e) => log_error!("Failed to create window: {}", e),
                }
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
            log_error!("Failed to connect to database: {}", e);
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
    application_state: State<ApplicationState>,
) -> Result<IPCResponse<Vec<StoredConnection>>, ()> {
    log_function!(get_saved_connections);
    match application_state.connection_storage.get_all_connections() {
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
    conn_name: String,
    project_id: String,
    application_state: State<ApplicationState>,
) -> Result<IPCResponse<String>, ()> {
    log_function!(delete_saved_connection);
    match application_state
        .connection_storage
        .delete_connection(&conn_name, &project_id)
    {
        Ok(f) => Ok(IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: None,
            sys_err: None,
            frontend_msg: Some(f),
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
fn fetch_dashboard_data(
    application_state: State<ApplicationState>,
    req_payload: DashboardDataRequest,
) -> IPCResponse<DashboardData> {
    log_function!(fetch_dashboard_data);
    log_info!("Fetching dashboard data and configuration");

    // Get connection data
    let connection_map = application_state.active_connection_map.lock().unwrap();
    let connection_data = match connection_map.get(&req_payload.connection_window_label) {
        Some(data) => data,
        None => {
            log_error!(
                "Connection data not found for window label: {}",
                req_payload.connection_window_label
            );
            return IPCResponse {
                status: http::status::StatusCode::OK.as_u16(),
                error_code: Some(constants::ERR_CODE_INVALID_CONN_DATA.to_string()),
                sys_err: Some("Connection data not found.".to_string()),
                frontend_msg: Some(
                    "Failed to retrieve connection details. Please try reconnecting.".to_string(),
                ),
                data: None,
            };
        }
    };

    let mut map = HashMap::new();
    map.insert("id".to_string(), connection_data.id.clone());
    map.insert("conn_name".to_string(), connection_data.conn_name.clone());
    map.insert("host_name".to_string(), connection_data.host_name.clone());
    map.insert(
        "database_name".to_string(),
        connection_data.database_name.clone(),
    );
    map.insert(
        "database_type".to_string(),
        connection_data.database_type.clone(),
    );
    map.insert("port".to_string(), connection_data.port.to_string());
    map.insert("user_name".to_string(), connection_data.user_name.clone());

    // Fetch schemas and tables
    let mut database_schemas: Vec<SchemaData> = Vec::new();
    let schemas_result = tauri::async_runtime::block_on(async {
        application_state
            .dbpool
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .fetch_schemas()
            .await
    });

    match schemas_result {
        Ok(schemas) => {
            log_info!("Successfully fetched {} schemas", schemas.len());
            for schema_name in schemas {
                let mut schema_tables: Vec<SchemaData> = Vec::new();
                let tables_result = tauri::async_runtime::block_on(async {
                    application_state
                        .dbpool
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .fetch_tables(&schema_name)
                        .await
                });

                match tables_result {
                    Ok(tables) => {
                        for table in tables {
                            schema_tables.push(SchemaData {
                                entity_type: "Table".to_string(),
                                entity_name: table.table_name,
                                is_expanded: false,
                                children: None,
                            });
                        }
                    }
                    Err(e) => {
                        log_error!("Failed to fetch tables for schema {}: {}", schema_name, e);
                        // Optionally handle error: return an error response or log and continue
                    }
                }
                database_schemas.push(SchemaData {
                    entity_type: "Schema".to_string(),
                    entity_name: schema_name,
                    is_expanded: true,
                    children: Some(schema_tables),
                });
            }
        }
        Err(e) => {
            log_error!("Failed to fetch schemas: {}", e);
            return IPCResponse {
                status: http::status::StatusCode::OK.as_u16(),
                error_code: Some(constants::ERR_CODE_DATABASE_FETCH_TABLES_FAILED.to_string()), // This error code might need to be adjusted or a new one created for schema fetching errors
                sys_err: Some(e.to_string()),
                frontend_msg: Some(e.to_string()),
                data: None,
            };
        }
    }

    // Construct the top-level database entity
    let mut dashboard_data = vec![SchemaData {
        entity_type: constants::POSTGRES_DATABASE_TYPE.to_string(),
        entity_name: {
            let mut chars = connection_data.database_name.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        },
        is_expanded: true,
        children: Some(database_schemas),
    }];

    // Add consoles entity if there are consoles present
    match application_state.sql_console_storage.list_console_files() {
        Ok(console_files) => {
            if !console_files.is_empty() {
                let children: Vec<SchemaData> = console_files
                    .into_iter()
                    .map(|file_name| SchemaData {
                        entity_type: "Console".to_string(),
                        entity_name: file_name,
                        is_expanded: false,
                        children: None,
                    })
                    .collect();

                let consoles_entity = SchemaData {
                    entity_type: "Consoles".to_string(),
                    entity_name: "Consoles".to_string(),
                    is_expanded: true,
                    children: Some(children),
                };
                dashboard_data.push(consoles_entity);
            }
        }
        Err(e) => {
            log_error!("Failed to list console files: {}", e);
            // Optionally return an error response or log and continue
        }
    }

    IPCResponse {
        status: http::status::StatusCode::OK.as_u16(),
        error_code: None,
        sys_err: None,
        frontend_msg: Some("Operation Successful".to_string()),
        data: Some(DashboardData {
            connection_data: map,
            dashboard_data,
        }),
    }
}

#[tauri::command]
fn fetch_table_data(
    req_payload: TableDataRequest,
    application_state: State<ApplicationState>,
) -> IPCResponse<TableData<String>> {
    log_function!(fetch_table_data);
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
                columns,
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
    log_function!(fetch_table_data_with_offset);
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

#[tauri::command]
fn save_console_file_cmd(
    file_path: String,
    content: String,
    application_state: State<ApplicationState>,
) -> Result<IPCResponse<String>, ()> {
    log_function!(save_console_file_cmd);
    match application_state.sql_console_storage.save_console_file(&file_path, &content) {
        Ok(f) => Ok(IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: None,
            sys_err: None,
            frontend_msg: Some("Console file saved successfully".to_string()),
            data: None,
        }),
        Err(e) => Ok(IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: Some(constants::ERR_CODE_STORAGE_FAILED.to_string()),
            sys_err: Some(e.to_string()),
            frontend_msg: Some("Failed to save console file".to_string()),
            data: None,
        }),
    }
}

#[tauri::command]
fn read_console_file_cmd(
    file_path: String,
    application_state: State<ApplicationState>,
) -> Result<IPCResponse<String>, ()> {
    log_function!(read_console_file_cmd);
    match application_state.sql_console_storage.read_console_file(&file_path) {
        Ok(content) => Ok(IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: None,
            sys_err: None,
            frontend_msg: Some(content),
            data: None,
        }),
        Err(e) => Ok(IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: Some(constants::ERR_CODE_STORAGE_FAILED.to_string()),
            sys_err: Some(e.to_string()),
            frontend_msg: Some("Failed to read console file".to_string()),
            data: None,
        }),
    }
}

#[tauri::command]
fn list_console_files_cmd(
    application_state: State<ApplicationState>,
) -> Result<IPCResponse<Vec<String>>, ()> {
    log_function!(list_console_files_cmd);
    match application_state.sql_console_storage.list_console_files() {
        Ok(files) => Ok(IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: None,
            sys_err: None,
            frontend_msg: None,
            data: Some(files),
        }),
        Err(e) => Ok(IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: Some(constants::ERR_CODE_STORAGE_FAILED.to_string()),
            sys_err: Some(e.to_string()),
            frontend_msg: Some("Failed to list console files".to_string()),
            data: None,
        }),
    }
}

#[tauri::command]
fn delete_console_file_cmd(
    file_path: String,
    application_state: State<ApplicationState>,
) -> Result<IPCResponse<String>, ()> {
    log_function!(delete_console_file_cmd);
    match application_state.sql_console_storage.delete_console_file(&file_path) {
        Ok(f) => Ok(IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: None,
            sys_err: None,
            frontend_msg: Some("Console file deleted successfully".to_string()),
            data: None,
        }),
        Err(e) => Ok(IPCResponse {
            status: http::status::StatusCode::OK.as_u16(),
            error_code: Some(constants::ERR_CODE_STORAGE_FAILED.to_string()),
            sys_err: Some(e.to_string()),
            frontend_msg: Some("Failed to delete console file".to_string()),
            data: None,
        }),
    }
}

fn main() {
    // Initialize logger
    if let Err(e) = logging::init_logger() {
        eprintln!("Failed to initialize logger: {}", e);
    }

    log_info!("Starting DataSquirrel application");

    // Initialize config manager
    let config_manager = config::ConfigManager::new().expect("Failed to initialize config manager");
    let connections_path = config_manager.get_config_path(&config::ConfigType::Connections);

    if connections_path.exists() {
        // Try to read the file to check if it's malformed
        if let Err(e) = std::fs::read_to_string(&connections_path)
            .and_then(|content| Ok(serde_json::from_str::<Vec<StoredConnection>>(&content)?))
        {
            // Create backup with timestamp
            let backup_path = connections_path.with_extension(format!(
                "{}.bak",
                chrono::Local::now().format("%Y%m%d_%H%M%S")
            ));
            std::fs::rename(&connections_path, &backup_path).expect("Failed to create backup");

            // Clean up old backups (keep only 2 most recent)
            let backup_dir = connections_path.parent().unwrap();
            let mut backups: Vec<_> = std::fs::read_dir(backup_dir)
                .unwrap()
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry
                        .path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext.starts_with("bak"))
                        .unwrap_or(false)
                })
                .collect();

            backups.sort_by(|a, b| {
                b.path()
                    .metadata()
                    .unwrap()
                    .modified()
                    .unwrap()
                    .cmp(&a.path().metadata().unwrap().modified().unwrap())
            });

            for old_backup in backups.into_iter().skip(2) {
                std::fs::remove_file(old_backup.path()).ok();
            }
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .manage(ApplicationState {
            dbpool: Mutex::new(None),
            connection_storage: ConnectionStorage::new(),
            active_connection_map: Mutex::new(HashMap::new()),
            sql_console_storage: sql_console_storage::SqlConsoleStorage::new().expect("Failed to initialize SQL console storage"),
        })
        .invoke_handler(tauri::generate_handler![
            init_connection,
            get_saved_connections,
            delete_saved_connection,
            fetch_dashboard_data,
            fetch_table_data,
            fetch_table_data_with_offset,
            save_console_file_cmd,
            read_console_file_cmd,
            list_console_files_cmd,
            delete_console_file_cmd,
        ])
        .setup(|app| {
            log_info!("Application setup started");

            // Get the main window that was created by the configuration
            if let Some(window) = app.get_webview_window("main") {
                // set background color only when building for macOS
                #[cfg(target_os = "macos")]
                {
                    use cocoa::appkit::{NSColor, NSWindow};
                    use cocoa::base::{id, nil};

                    let ns_window = window.ns_window().unwrap() as id;
                    unsafe {
                        let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                            nil,
                            50.0 / 255.0,
                            158.0 / 255.0,
                            163.5 / 255.0,
                            1.0,
                        );
                        ns_window.setBackgroundColor_(bg_color);
                    }
                }
            }

            // Your setup code here
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
