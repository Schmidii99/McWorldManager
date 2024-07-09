// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;

use std::default;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use base64::{engine::general_purpose, Engine as _};

use state::{AppState, ServiceAccess};
use tauri::api::path;
use tauri::{State, Manager, AppHandle};

use rust_search::{SearchBuilder, similarity_sort};
use platform_dirs::AppDirs;

fn main() {
  tauri::Builder::default()
      .manage(AppState { db: Default::default() })
      .invoke_handler(tauri::generate_handler![greet, get_saves])
      .setup(|app| {
          let handle = app.handle();

          let app_state: State<AppState> = handle.state();
          let db = database::initialize_database(&handle).expect("Database initialize should succeed");
          *app_state.db.lock().unwrap() = Some(db);

          Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

#[tauri::command]
fn greet(app_handle: AppHandle, name: &str) -> String {
    // Should handle errors instead of unwrapping here
    app_handle.db(|db| database::add_item(name, db)).unwrap();

    let items = app_handle.db(|db| database::get_all(db)).unwrap();

    let items_string = items.join(" | ");

    format!("Your name log: {}", items_string)
}

#[tauri::command]
fn get_saves(_app_handle: AppHandle) -> String {
    let minecraft_folder_path: PathBuf = AppDirs::new(Some(".minecraft\\saves"), true).unwrap().config_dir;

    let paths = fs::read_dir(minecraft_folder_path).unwrap();

    let mut saves: String = String::from("");

    for p in paths {
        if p.as_ref().unwrap().metadata().unwrap().is_dir() {

            let region_path = p.as_ref().unwrap().path().join("region");
            // println!("{}", region_path.display());

            // check if region dir exists
            if region_path.exists() {
                saves += &(general_purpose::STANDARD.encode(p.as_ref().unwrap().path().display().to_string()) + ",");
            }
        }    
    }

    saves
}