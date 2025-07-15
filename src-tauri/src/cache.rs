#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{State, command};
use tauri_plugin_sql::{Migration, Sql};
use zstd::stream::{compress, decompress_all};
use rusqlite::Connection;

/// Shared DB handle
pub struct CacheDB(Mutex<Connection>);

#[derive(Serialize, Deserialize)]
pub struct RowData {
    pub id: String,
    pub data: serde_json::Value,
}

#[tauri::command]
pub fn init_cache_db(state: State<CacheDB>) -> Result<(), String> {
    let conn = state.0.lock().unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cache_entries (
      tab_id TEXT NOT NULL,
      row_idx INTEGER NOT NULL,
      data_blob BLOB NOT NULL,
      PRIMARY KEY (tab_id, row_idx)
    )",
        [],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Save one row compressed into SQLite
#[command]
async fn save_cache_entry(
    db: State<'_, Db>,
    tab_id: String,
    row_idx: u32,
    row_json: String,
) -> Result<(), String> {
    let blob = compress(row_json.as_bytes(), 0).map_err(|e| e.to_string())?;
    db.0.execute(
        "REPLACE INTO cache_entries (tab_id, row_idx, data_blob) VALUES (?1,?2,?3)",
        [tab_id.as_str(), &row_idx.to_string(), &blob],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Retrieve and decompress one row
#[command]
async fn get_cache_entry(
    db: State<'_, Db>,
    tab_id: String,
    row_idx: u32,
) -> Result<Option<String>, String> {
    let mut stmt =
        db.0.prepare("SELECT data_blob FROM cache_entries WHERE tab_id = ?1 AND row_idx = ?2")
            .map_err(|e| e.to_string())?;
    let mut rows = stmt
        .query([tab_id.as_str(), &row_idx.to_string()])
        .map_err(|e| e.to_string())?;
    if let Some(row) = rows.next().transpose().map_err(|e| e.to_string())? {
        let blob: Vec<u8> = row.get(0).map_err(|e| e.to_string())?;
        let decompressed = decompress_all(&blob).map_err(|e| e.to_string())?;
        let s = String::from_utf8(decompressed).map_err(|e| e.to_string())?;
        Ok(Some(s))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub fn clear_cache(tab_id: Option<String>, state: State<CacheDB>) -> Result<(), String> {
  let conn = state.0.lock().unwrap();
  match tab_id {
    Some(id) => conn.execute("DELETE FROM cache_entries WHERE tab_id = ?1", params![id]),
    None => conn.execute("DELETE FROM cache_entries", []),
  }
  .map_err(|e| e.to_string())?;
  Ok(())
}
