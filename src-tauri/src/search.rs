use everything_sdk::*;
use base64::{engine::general_purpose, Engine as _};

/// Returns a list of base64 encoded world folder paths.
pub fn find_world_paths_with_everything() -> Result<Vec<String>> {
    let mut everything = global().try_lock().unwrap();

    let mut saves: Vec<String> = Vec::new();

    // Check whether the Everything.exe in the background is running.
    match everything.is_db_loaded() {
        Ok(false) => panic!("The Everything database has not been fully loaded now."),
        Err(EverythingError::Ipc) => panic!("Everything is required to run in the background."),
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
                let full_path = item.filepath().unwrap();

                saves.push(general_purpose::STANDARD.encode(full_path.display().to_string()) + ",");
            }

            Ok(saves)
        }
    }

    
}