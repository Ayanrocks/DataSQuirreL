#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::types::api_objects::ApplicationState;
use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;
use tauri::State;
use tokio::sync::Mutex;
use zstd::{decode_all, encode_all};

/// Shared DB handle
pub struct CacheDB(pub Mutex<SqliteConnection>);

#[derive(Serialize, Deserialize)]
pub struct RowData {
    pub id: String,
    pub data: serde_json::Value,
}

#[tauri::command]
pub async fn init_cache_db(state: State<'_, ApplicationState>) -> Result<(), String> {
    let mut conn = state.sqlite_db.0.lock().await;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS cache_entries (
      tab_id TEXT NOT NULL,
      row_idx INTEGER NOT NULL,
      data_blob BLOB NOT NULL,
      PRIMARY KEY (tab_id, row_idx)
    )",
    )
    .execute(&mut *conn)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Save one row compressed into SQLite
#[tauri::command]
pub async fn save_cache_entry(
    tab_id: String,
    row_idx: u32,
    row_json: String,
    state: State<'_, ApplicationState>,
) -> Result<(), String> {
    let mut db = state.sqlite_db.0.lock().await;
    let blob = encode_all(row_json.as_bytes(), 0).map_err(|e| e.to_string())?;
    sqlx::query("REPLACE INTO cache_entries (tab_id, row_idx, data_blob) VALUES (?, ?, ?)")
        .bind(tab_id)
        .bind(row_idx)
        .bind(blob)
        .execute(&mut *db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Retrieve and decompress one row
#[tauri::command]
pub async fn get_cache_entry(
    state: State<'_, ApplicationState>,
    tab_id: String,
    row_idx: u32,
) -> Result<Option<String>, String> {
    let mut conn = state.sqlite_db.0.lock().await;
    let row: Option<(Vec<u8>,)> =
        sqlx::query_as("SELECT data_blob FROM cache_entries WHERE tab_id = ? AND row_idx = ?")
            .bind(tab_id)
            .bind(row_idx)
            .fetch_optional(&mut *conn)
            .await
            .map_err(|e| e.to_string())?;

    if let Some((blob,)) = row {
        let decompressed = decode_all(&blob[..]).map_err(|e| e.to_string())?;
        let s = String::from_utf8(decompressed).map_err(|e| e.to_string())?;
        Ok(Some(s))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn clear_cache(
    tab_id: Option<String>,
    state: State<'_, ApplicationState>,
) -> Result<(), String> {
    let mut conn = state.sqlite_db.0.lock().await;
    match tab_id {
        Some(id) => {
            sqlx::query("DELETE FROM cache_entries WHERE tab_id = ?")
                .bind(id)
                .execute(&mut *conn)
                .await
                .map_err(|e| e.to_string())?;
        }
        None => {
            sqlx::query("DELETE FROM cache_entries")
                .execute(&mut *conn)
                .await
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
