use std::fs;
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::io::ErrorKind;

use rusqlite::{Connection, Result};

const DB_NAME: &'static str = "automations.db";

#[derive(PartialEq, Debug)]
pub enum DBError {
    InitialisationError(&'static str),
}

pub fn init(path: &str) -> Result<(), DBError> {
    /* Getting the path to the home directory */
    let home_path: OsString;

    match env::var_os("HOME") {
        Some(val) => {
            home_path = val;
        },
        _ => return Err(DBError::InitialisationError("Could not find home directory")),
    }

    /* Creating the data directory */
    let mut absolute_path = PathBuf::from(home_path);
    absolute_path.push(path);

    let creation_result = fs::create_dir(&absolute_path);
    
    match creation_result {
        Ok(_) => {},
        Err(e) => {
            match e.kind() {
                ErrorKind::AlreadyExists => {}, // We don't mind if the directory already exists
                _ => return Err(DBError::InitialisationError("Error creating the data directory")),
            }
        },
    };

    /* Initialising the database, if this is the first time the daemon is started. */
    let mut db_path = absolute_path.clone();
    db_path.push(&DB_NAME);

    let sql_init = init_sqlite(&db_path);

    match sql_init {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
            return Err(DBError::InitialisationError("Error initialising the database"));
        }
    }

    Ok(())
}

pub fn init_sqlite(db_path: &PathBuf) -> Result<()> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS automations (
                id INTEGER NOT NULL PRIMARY KEY,
                name TEXT NOT NULL DEFAULT 'Unnamed Automation',
                active INTEGER NOT NULL DEFAULT 1
            );",
        ()
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS actions (
                id INTEGER NOT NULL PRIMARY KEY,
                automation_id INTEGER NOT NULL,
                execution_index INT NOT NULL,
                action TEXT NOT NULL,
                
                action_details TEXT NOT NULL DEFAULT '{}',

                UNIQUE(automation_id, execution_index),
                FOREIGN KEY (automation_id) REFERENCES automations(id) ON DELETE CASCADE
            );",
        ()
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS triggers (
                id INTEGER NOT NULL PRIMARY KEY,
                automation_id INTEGER NOT NULL,
                type TEXT NOT NULL,
                
                trigger_details TEXT NOT NULL DEFAULT '{}',
                
                FOREIGN KEY (automation_id) REFERENCES automations(id) ON DELETE CASCADE
            );",
        ()
    )?;

    Ok(())
}

pub fn add_automation(db_path: &PathBuf, name: String) -> Result<()> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "INSERT INTO automations (name) VALUES (?1)",
        rusqlite::params![&name],
    )?;

    Ok(())
}
