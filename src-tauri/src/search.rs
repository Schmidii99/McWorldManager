use std::path::PathBuf;

use everything_sdk::*;

pub struct PathsFound {
    pub world_paths: Vec<PathBuf>,
    pub server_paths: Vec<PathBuf>,
    pub invalid_path_count: u32,
}

/// Returns a list of base64 encoded world folder paths.
pub fn find_world_paths_with_everything() -> Result<PathsFound> {
    let mut everything = global().try_lock().unwrap();

    let mut worlds: Vec<PathBuf> = Vec::new();
    let mut servers: Vec<PathBuf> = Vec::new();
    let mut invalid_count = 0;

    // Check whether the Everything.exe in the background is running.
    match everything.is_db_loaded() {
        Ok(false) => Err(EverythingError::InvalidCall),
        Err(EverythingError::Ipc) => Err(EverythingError::Ipc),
        _ => {
            // Now _Everything_ is OK!

            // We got the searcher, which can be reused for multiple times queries and cleans up
            // memory when it has been dropped.
            let mut searcher = everything.searcher();

            // Set the query parameters, chaining call is optional.
            searcher.set_search("level.dat");
            searcher
                .set_request_flags(
                    RequestFlags::EVERYTHING_REQUEST_FILE_NAME
                        | RequestFlags::EVERYTHING_REQUEST_PATH
                        | RequestFlags::EVERYTHING_REQUEST_SIZE
                        // | RequestFlags::EVERYTHING_REQUEST_ATTRIBUTES // no attr-data request
                        | RequestFlags::EVERYTHING_REQUEST_RUN_COUNT,
                )
                .set_match_whole_word(true)	
                .set_sort(SortType::EVERYTHING_SORT_DATE_RECENTLY_CHANGED_ASCENDING);

            // Send IPC query now, _block_ and wait for the result to return.
            // Some heavy query (like search single 'a') may take a lot of time in IPC data transfer, so
            // if you need unblocking, do them in a new thread or enable the `async` feature in crate.
            let results = searcher.query();

            // Make sure you set the corresponding `RequestFlags` for getting result props.

            // Walking the 5 query results from Everything IPC by iterator.
            for item in results.iter() {
                let mut full_path = item.filepath().unwrap();
                full_path.pop();

                let mut parent_path = full_path.clone();
                // check if region folder exists
                let region_path = PathBuf::from(parent_path.display().to_string() + "\\region\\");
                if !region_path.exists() { 
                    println!("Skipping due to no region folder: {}", full_path.display());
                    invalid_count += 1;
                    continue; 
                }
                parent_path.pop();
                // check if folder is a server path then skip
                let eula_path = PathBuf::from(parent_path.display().to_string() + "\\eula.txt");
                if eula_path.exists() { 
                    println!("Skipping due to path linking to a server: {}", full_path.display());
                    servers.push(full_path);
                    continue; 
                }

                println!("Found valid world path: {}", full_path.display());

                worlds.push(full_path);
            }

            Ok(PathsFound { world_paths: worlds, server_paths: servers, invalid_path_count: invalid_count })
        }
    }

    
}