use rusqlite::{Connection, named_params};
use tauri::AppHandle;
use std::fs;
use base64::{engine::general_purpose, Engine as _};

// Holds a list of database instructions used when updating to a new version of the database.
// Entry 1 = first database version, and so on
const DATABASE_INSTRUCTIONS: [&str; 1] = [
  "CREATE TABLE IF NOT EXISTS world_paths (
    id INTEGER NOT NULL,
    path TEXT NOT NULL,
    PRIMARY KEY('id')
  );",
];

/// Initializes the database connection, creating the .sqlite file if needed, and upgrading the database
/// if it's out of date.
pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle.path_resolver().app_data_dir().expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join("McWorldManager.sqlite");

    println!("Database path: {}", sqlite_path.display());

    let mut db = Connection::open(sqlite_path)?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| { Ok(row.get(0)?) })?;
    drop(user_pragma);

    upgrade_database_if_needed(&mut db, existing_user_version)?;

    Ok(db)
}

/// Upgrades the database to the current version.
pub fn upgrade_database_if_needed(db: &mut Connection, existing_version: u32) -> Result<(), rusqlite::Error> {
  if existing_version as usize == DATABASE_INSTRUCTIONS.len() {
    return Ok(());
  }

  let mut current_version: usize = existing_version as usize;

  while current_version < DATABASE_INSTRUCTIONS.len() {
    let tx = db.transaction()?;

    // execute instructions for the next version
    tx.execute_batch(DATABASE_INSTRUCTIONS[current_version])?;
    current_version += 1;
    tx.pragma_update(None, "user_version", current_version)?;

    tx.commit()?;

    println!("Database updated to version {}", current_version);
  }

  Ok(())
}

pub fn add_path(path: &str, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare("INSERT INTO world_paths (path) VALUES (@path)")?;
    statement.execute(named_params! { "@path": general_purpose::STANDARD.encode(path) })?;

    Ok(())
}

pub fn get_all_paths(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT * FROM world_paths")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
      let path: String = row.get("path")?;

      items.push(path);
    }
  
    Ok(items)
}

use std::path::PathBuf;

pub fn insert_subfolders_of_folder(db: &Connection, saves_path: PathBuf) {
  let paths = fs::read_dir(saves_path).unwrap();

  for p in paths {
      match p {
          Ok(entry) => {
            // check if folder has region sub folder otherwise skip
            if !(entry.path().join("region").exists()) { continue; }

            println!("Adding path: {}", entry.path().to_str().unwrap());

            // check if entry does not exists
            let mut statement = db.prepare("SELECT * FROM world_paths WHERE path = @path").unwrap();
            let mut rows = statement.query(named_params! { "@path": general_purpose::STANDARD.encode(entry.path().to_str().unwrap()) }).unwrap();

            // check if no rows were returned
            let res = rows.next();
            match res {
                Ok(opt) => {
                  // None means there are no rows with this path
                  if opt.is_none() {
                    add_path(entry.path().to_str().unwrap(), db).unwrap();
                  }
                },
                _ => { continue; }
            }
          },
          Err(_e) => continue
      }   
  }
}

pub fn get_path_count(db: &Connection) -> Result<usize, rusqlite::Error> {
    let mut statement = db.prepare("SELECT COUNT(*) FROM world_paths")?;
    let mut rows = statement.query([])?;
    let mut count = 0;
    while let Some(row) = rows.next()? {
      count = row.get(0)?;
    }
  
    Ok(count)
}
