// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod keyboard;

use std::thread;
use tauri_plugin_autostart::MacosLauncher;
use rusqlite::Connection;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct result_vector_value{
    trigger: String,
    value: String
}
fn create_or_check_file() -> std::io::Result<()> {
  let file_path = "../Data.db";
  let path = Path::new(file_path);
  if path.exists() {
      
  } else {
      let mut file = fs::File::create(file_path)?;
  }
  Ok(())
}
#[tauri::command]
fn get_all_data() -> Vec<result_vector_value> {
  let conn = Connection::open("../Data.db").unwrap();
  conn.execute(
      "CREATE TABLE IF NOT EXISTS triggervalue (trigger TEXT UNIQUE NOT NULL, value TEXT NOT NULL)",
      (),
  )
  .unwrap();
  let mut stmt = conn.prepare("SELECT trigger, value FROM triggervalue").unwrap();
  let mut result_vector: Vec<result_vector_value> = Vec::new();
  let data_iter = stmt.query_map([], |row| {
      Ok(result_vector_value {
          trigger: row.get(0)?,
          value: row.get(1)?,
      })
  })
  .unwrap();
  for data in data_iter {
      result_vector.push(data.unwrap());
  }
  result_vector
}
#[tauri::command]
fn delete_by_trigger(trigger: &str) -> () {
  let conn = Connection::open("../Data.db").unwrap();
  conn.execute(
      "CREATE TABLE IF NOT EXISTS triggervalue (trigger TEXT UNIQUE NOT NULL, value TEXT NOT NULL)",
      (),
  )
  .unwrap();
  conn.execute(
      "DELETE FROM triggervalue WHERE trigger=:trigg",
      &[(":trigg", trigger)],
  )
  .unwrap();
}
#[tauri::command]
fn insert_data(trigger: &str, value: &str) -> () {
  let conn = Connection::open("../Data.db").unwrap();
  conn.execute(
      "CREATE TABLE IF NOT EXISTS triggervalue (trigger TEXT UNIQUE NOT NULL, value TEXT NOT NULL)",
      (),
  )
  .unwrap();
  conn.execute(
      "INSERT INTO triggervalue (trigger, value) VALUES (:trigg, :val)",
      &[(":trigg", trigger), (":val", value)],
  )
  .unwrap();
}
#[tauri::command]
fn update_by_trigger(trigger: &str, value: &str) -> () {
  let conn = Connection::open("../Data.db").unwrap();
  conn.execute(
      "CREATE TABLE IF NOT EXISTS triggervalue (trigger TEXT UNIQUE NOT NULL, value TEXT NOT NULL)",
      (),
  )
  .unwrap();
  conn.execute(
      "UPDATE triggervalue SET value=:val WHERE trigger=:trigg",
      &[(":trigg", trigger), (":val", value)],
  )
  .unwrap();
}
#[tauri::command]
fn delete_all_data()->(){
  let conn = Connection::open("../Data.db").unwrap();
  conn.execute(
      "CREATE TABLE IF NOT EXISTS triggervalue (trigger TEXT UNIQUE NOT NULL, value TEXT NOT NULL)",
      (),
  )
  .unwrap();
  conn.execute(
      "DELETE FROM triggervalue",
      [],
  )
  .unwrap();
}
fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"])))
    .invoke_handler(tauri::generate_handler![get_all_data,delete_by_trigger,insert_data,update_by_trigger,delete_all_data])
    .setup(|app|{
      create_or_check_file().unwrap();
      let tauri_app_handle=app.handle();
      thread::spawn(move||{
        keyboard::keyboard_listener(tauri_app_handle);
      });
      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|_app_handle, event| match event {
      tauri::RunEvent::ExitRequested { api, .. } => {
         api.prevent_exit();
      }
      _ => {}
    });
}