//! This module contains the main entrypoint to the daemon.
//! Initialises the database and the API.

use std::path::PathBuf;

use zbus::blocking::connection;

use crate::api::AutomationAPI;

mod db;
mod runner;
mod api;

/// The directory at which all data is held (relative to the
/// home directory).
const DATA_DIRECTORY: &str = ".sigroute/";

/// The entrypoint to the daemon.
fn main() {
    let result = db::init(DATA_DIRECTORY);

    match result {
        Ok(db_path) => {
            println!("[Info] - sigrouted running...");

            let _ = run_api(db_path);
        }
        Err(_) => {
            println!("Error: could not initialise sigroute database.");
            return;
        }
    }
}

/// Initialises and runs the API in this thread.
/// Warning: this code does NOT return.
fn run_api(db_path: PathBuf) -> zbus::Result<()> {
    let automation_api = AutomationAPI::new(db_path);
    let _connection = connection::Builder::session()?
        .name("uk.co.sebcrookes.Sigroute")?
        .serve_at("/uk/co/sebcrookes/Sigroute", automation_api)?
        .build()?;

    loop {
        std::thread::park();
    }

    #[allow(unreachable_code)]
    Ok(())
}
