// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;
mod search;
mod nbt_reader;

use std::{fs, path::PathBuf};

use everything_sdk::EverythingError;
use nbt::Blob;
use rusqlite::Connection;
use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle, CustomMenuItem, Menu, Submenu};

use native_dialog::FileDialog;
use platform_dirs::AppDirs;

// event payload
#[derive(Clone, serde::Serialize)]
struct MessageboxPayload {
  message: String,
  title: String
}

fn main() {
    // create menus
    let reload_default: CustomMenuItem = CustomMenuItem::new("reload_default".to_string(), "Reload default .minecraft path");
    let add_worlds_folder: CustomMenuItem = CustomMenuItem::new("add_world_folder".to_string(), "Add worlds folder");
    let find_everything: CustomMenuItem = CustomMenuItem::new("find_everything".to_string(), "Find worlds with Everything.exe");
    let submenu: Submenu = Submenu::new("Add Sources", Menu::new().add_item(reload_default).add_item(add_worlds_folder).add_item(find_everything));
    let menu: Menu = Menu::new().add_submenu(submenu);

    tauri::Builder::default()
      .menu(menu)
      .on_menu_event(|event| {
        let app_handle: AppHandle = event.window().app_handle();

        match event.menu_item_id() {
          "reload_default" => {
            std::process::exit(0);
          }
          "add_world_folder" => {
            let path: String = open_folder_picker();
            if path.is_empty() { return; }

            app_handle.db(|db: &Connection| database::insert_subfolders_of_folder(db, PathBuf::from(path)));
          }
          "find_everything" => {
            let paths_found = search::find_world_paths_with_everything();
            match paths_found {
                Ok(paths) => {
                  for path in &paths.world_paths {
                    app_handle.db(|db: &Connection| database::insert_world_folder(db, path));
                  }

                  app_handle.emit_all("messagebox", MessageboxPayload {message: "Success!\n Found ".to_owned() + &paths.world_paths.len().to_string() + " worlds \nSkipped " + &paths.invalid_path_count.to_string() + " invalid paths.", title: "Everything search".to_string()}).unwrap();
                },
                Err(EverythingError::Ipc) => {
                  app_handle.emit_all("messagebox_error", MessageboxPayload {message: "Everything is required to run in the background.".to_string(), title: "Everything search".to_string()}).unwrap();
                }
                Err(EverythingError::InvalidCall) => {
                  app_handle.emit_all("messagebox_error", MessageboxPayload {message: "The Everything database has not been fully loaded now.".to_string(), title: "Everything search".to_string()}).unwrap();
                }
                _ => {}
            }
          }
          _ => {}
        }
        // trigger refresh event in frontend
        app_handle.emit_all("refresh", {}).unwrap();

      })
      .manage(AppState { db: Default::default() })
      .invoke_handler(tauri::generate_handler![deserialize_nbt_file, get_saved_paths])
      .setup(|app: &mut tauri::App| {
            let handle: AppHandle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db: Connection = database::initialize_database(&handle).expect("Database initialize should succeed");
          
            add_default_minecraft_paths(&db);
          
            *app_state.db.lock().unwrap() = Some(db);
            Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

fn open_folder_picker() -> String {
    let path: Option<PathBuf> = FileDialog::new()
        .set_location("~/Desktop")
        .show_open_single_dir()
        .unwrap();

    match path {
        Some(path) => return path.display().to_string(),
        None => return String::from(""),
    };
}

fn add_default_minecraft_paths(db: &Connection) {
    if database::get_path_count(db).unwrap() != 0 {
        return;
    }

    let default_minecraft_saves_path: PathBuf = AppDirs::new(Some(".minecraft\\saves"), true).unwrap().config_dir;
    database::insert_subfolders_of_folder(db, default_minecraft_saves_path);
}

#[tauri::command]
fn deserialize_nbt_file(path: &str) -> String {
    let mut input_file: fs::File = std::fs::File::open(path).unwrap();
    
    let data_blob: Blob = nbt_reader::detect_and_read_nbt(&mut input_file).unwrap_or(Blob::default());

    serde_json::to_string(&data_blob).unwrap()
}

#[tauri::command]
fn get_saved_paths(app_handle: AppHandle) -> String {
    let paths: Vec<String> = app_handle.db(|db: &Connection| database::get_all_paths(db)).unwrap();

    paths.join(",")
}