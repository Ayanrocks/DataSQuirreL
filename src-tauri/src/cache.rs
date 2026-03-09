#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::types::api_objects::ApplicationState;
use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;
use tauri::State;
use tokio::sync::Mutex;
use zstd::{decode_all, encode_all};

/// Shared DB handle
pub struct CacheDB(pub Mutex<SqliteConnection>);

pub const CACHE_ENTRIES_DDL: &str = "\
CREATE TABLE IF NOT EXISTS cache_entries (
  tab_id TEXT NOT NULL,
  row_idx INTEGER NOT NULL,
  data_blob BLOB NOT NULL,
  PRIMARY KEY (tab_id, row_idx)
)";

impl CacheDB {
    pub async fn init(&self) -> Result<(), String> {
        let mut conn = self.0.lock().await;
        sqlx::query(CACHE_ENTRIES_DDL)
            .execute(&mut *conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn save_entry(
        &self,
        tab_id: String,
        row_idx: u32,
        row_json: String,
    ) -> Result<(), String> {
        let blob = encode_all(row_json.as_bytes(), 0).map_err(|e| e.to_string())?;

        let mut db = self.0.lock().await;
        sqlx::query("REPLACE INTO cache_entries (tab_id, row_idx, data_blob) VALUES (?, ?, ?)")
            .bind(tab_id)
            .bind(row_idx)
            .bind(blob)
            .execute(&mut *db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_entry(&self, tab_id: String, row_idx: u32) -> Result<Option<String>, String> {
        let row: Option<(Vec<u8>,)> = {
            let mut conn = self.0.lock().await;
            sqlx::query_as("SELECT data_blob FROM cache_entries WHERE tab_id = ? AND row_idx = ?")
                .bind(tab_id)
                .bind(row_idx)
                .fetch_optional(&mut *conn)
                .await
                .map_err(|e| e.to_string())?
        };

        if let Some((blob,)) = row {
            let decompressed = decode_all(&blob[..]).map_err(|e| e.to_string())?;
            let s = String::from_utf8(decompressed).map_err(|e| e.to_string())?;
            Ok(Some(s))
        } else {
            Ok(None)
        }
    }

    pub async fn clear(&self, tab_id: Option<String>) -> Result<(), String> {
        let mut conn = self.0.lock().await;
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
}

#[derive(Serialize, Deserialize)]
pub struct RowData {
    pub id: String,
    pub data: serde_json::Value,
}

#[tauri::command]
pub async fn init_cache_db(state: State<'_, ApplicationState>) -> Result<(), String> {
    state.sqlite_db.init().await
}

/// Save one row compressed into SQLite
#[tauri::command]
pub async fn save_cache_entry(
    tab_id: String,
    row_idx: u32,
    row_json: String,
    state: State<'_, ApplicationState>,
) -> Result<(), String> {
    state.sqlite_db.save_entry(tab_id, row_idx, row_json).await
}

/// Retrieve and decompress one row
#[tauri::command]
pub async fn get_cache_entry(
    state: State<'_, ApplicationState>,
    tab_id: String,
    row_idx: u32,
) -> Result<Option<String>, String> {
    state.sqlite_db.get_entry(tab_id, row_idx).await
}

#[tauri::command]
pub async fn clear_cache(
    tab_id: Option<String>,
    state: State<'_, ApplicationState>,
) -> Result<(), String> {
    state.sqlite_db.clear(tab_id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Connection;

    async fn setup_db() -> CacheDB {
        let conn = SqliteConnection::connect("sqlite::memory:").await.unwrap();
        let db = CacheDB(Mutex::new(conn));
        db.init().await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_init_cache_db_creates_table() {
        let db = setup_db().await;
        // Verify table exists
        let mut conn = db.0.lock().await;
        let row: (i64,) = sqlx::query_as(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='cache_entries'",
        )
        .fetch_one(&mut *conn)
        .await
        .unwrap();
        assert_eq!(row.0, 1);
    }

    #[tokio::test]
    async fn test_save_and_get_cache_entry() {
        let db = setup_db().await;
        let tab_id = "test_tab".to_string();
        let row_idx = 1;
        let row_json = r#"{"id": "1", "data": "test"}"#.to_string();

        db.save_entry(tab_id.clone(), row_idx, row_json.clone())
            .await
            .unwrap();

        let result = db.get_entry(tab_id, row_idx).await.unwrap();
        assert_eq!(result.unwrap(), row_json);
    }

    #[tokio::test]
    async fn test_get_non_existent_entry_returns_none() {
        let db = setup_db().await;
        let result = db.get_entry("non_existent".to_string(), 1).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_clear_cache_with_tab_id() {
        let db = setup_db().await;
        db.save_entry("tab1".to_string(), 1, "data1".to_string())
            .await
            .unwrap();
        db.save_entry("tab1".to_string(), 2, "data2".to_string())
            .await
            .unwrap();
        db.save_entry("tab2".to_string(), 1, "data3".to_string())
            .await
            .unwrap();

        db.clear(Some("tab1".to_string())).await.unwrap();

        assert!(db.get_entry("tab1".to_string(), 1).await.unwrap().is_none());
        assert!(db.get_entry("tab1".to_string(), 2).await.unwrap().is_none());
        assert!(db.get_entry("tab2".to_string(), 1).await.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_clear_cache_all() {
        let db = setup_db().await;
        db.save_entry("tab1".to_string(), 1, "data1".to_string())
            .await
            .unwrap();
        db.save_entry("tab2".to_string(), 1, "data3".to_string())
            .await
            .unwrap();

        db.clear(None).await.unwrap();

        assert!(db.get_entry("tab1".to_string(), 1).await.unwrap().is_none());
        assert!(db.get_entry("tab2".to_string(), 1).await.unwrap().is_none());
    }
}
