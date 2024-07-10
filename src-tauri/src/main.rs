// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;

use std::{
    fs,
    fs::File,
    io::{self, Read, Seek},
};
use std::path::PathBuf;
use base64::{engine::general_purpose, Engine as _};

use nbt::Blob;
use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

use rust_search::SearchBuilder;
use platform_dirs::AppDirs;

fn main() {
  tauri::Builder::default()
      .manage(AppState { db: Default::default() })
      .invoke_handler(tauri::generate_handler![greet, get_saves, deserialize_nbt_file, find_world_paths])
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

#[tauri::command]
fn deserialize_nbt_file(path: &str) -> String {
    let mut input_file: fs::File = std::fs::File::open(path).unwrap();
    
    let data_blob = detect_and_read_nbt(&mut input_file).unwrap_or(Blob::default());

    serde_json::to_string(&data_blob).unwrap()
}

fn detect_and_read_nbt(input: &mut File) -> io::Result<nbt::Blob> {
    input.seek(std::io::SeekFrom::Start(0))?;
    let mut header = [0; 2];
    input.read_exact(&mut header)?;
    input.seek(std::io::SeekFrom::Start(0))?;
    match header {
        // Common ZLIB headers: https://stackoverflow.com/a/17176881
        [0x78, 0x01] | [0x78, 0x9C] | [0x78, 0xDA] => Ok(nbt::from_zlib_reader(input)?),
        // GZIP header: https://en.wikipedia.org/wiki/Gzip#File_format
        [0x1f, 0x8b] => Ok(nbt::from_gzip_reader(input)?),
        // Assume we have raw (uncompressed) NBT
        _ => Ok(nbt::from_reader(input)?),
    }
}

#[tauri::command]
fn find_world_paths() {
    todo!();
}