use std::path::PathBuf;

use zbus::blocking::connection;

use crate::api::AutomationAPI;

mod db;
mod runner;
mod api;

fn main() {
    let result = db::init(".sigroute/");

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

fn run_api(db_path: PathBuf) -> zbus::Result<()> {
    let automation_api = AutomationAPI::new(db_path);
    let _connection = connection::Builder::session()?
        .name("uk.co.sebcrookes.Sigroute")?
        .serve_at("/uk/co/sebcrookes/Sigroute", automation_api)?
        .build()?;

    std::thread::park();

    Ok(())
}
