// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;
mod search;
mod nbt_reader;

use std::fs;

use nbt::Blob;
use rusqlite::Connection;
use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

fn main() {
  tauri::Builder::default()
      .manage(AppState { db: Default::default() })
      .invoke_handler(tauri::generate_handler![deserialize_nbt_file, get_saved_paths])
      .setup(|app| {
          let handle = app.handle();

          let app_state: State<AppState> = handle.state();
          let db = database::initialize_database(&handle).expect("Database initialize should succeed");
          
          add_default_minecraft_paths(&db);
          
          *app_state.db.lock().unwrap() = Some(db);

          Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

fn add_default_minecraft_paths(db: &Connection) {
    if database::get_path_count(db).unwrap() != 0 {
        return;
    }

    database::insert_default_minecraft_folder(db);
}

#[tauri::command]
fn deserialize_nbt_file(path: &str) -> String {
    let mut input_file: fs::File = std::fs::File::open(path).unwrap();
    
    let data_blob = nbt_reader::detect_and_read_nbt(&mut input_file).unwrap_or(Blob::default());

    serde_json::to_string(&data_blob).unwrap()
}

#[tauri::command]
fn get_saved_paths(app_handle: AppHandle) -> String {
    let paths = app_handle.db(|db| database::get_all_paths(db)).unwrap();

    paths.join(",")
}