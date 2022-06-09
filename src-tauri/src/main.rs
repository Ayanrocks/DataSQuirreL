#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DBConnectionRequest {
  conn_name: String,
  host_name: String,
  port: i32,
  user_name: String,
}

#[tauri::command]
fn my_custom_command(req_payload: DBConnectionRequest) {
  println!("I was invoked from JS! {:?}", req_payload.conn_name);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
