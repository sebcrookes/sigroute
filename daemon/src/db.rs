use std::fs;
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::io::ErrorKind;

use rusqlite::{Connection, Result};
use sigroute_common::Automation;
use sigroute_common::AutomationAction;
use sigroute_common::AutomationTrigger;

const DB_NAME: &'static str = "automations.db";

#[derive(PartialEq, Debug)]
pub enum DBError {
    InitialisationError(&'static str),
}

pub fn init(path: &str) -> Result<PathBuf, DBError> {
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
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            return Err(DBError::InitialisationError("Error initialising the database"));
        }
    }

    Ok(db_path)
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
                execution_index INTEGER NOT NULL,
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
                type INTEGER NOT NULL,
                
                trigger_details TEXT NOT NULL DEFAULT '{}',
                
                FOREIGN KEY (automation_id) REFERENCES automations(id) ON DELETE CASCADE
            );",
        ()
    )?;

    Ok(())
}

pub fn add_automation(db_path: &PathBuf, name: String) -> Result<i64> {
    let conn = Connection::open(db_path)?;

    // Adding the new automation to the table (with a self-assigning automation ID)
    conn.execute(
        "INSERT INTO automations (name) VALUES (?1)",
        rusqlite::params![&name],
    )?;

    // Getting the automation ID of the added automation
    let id: i64 = conn.query_row(
        "SELECT id FROM automations WHERE rowid = ?1",
        rusqlite::params![conn.last_insert_rowid()],
        |row| {
            Ok(row.get(0))
        },
    )??;

    Ok(id)
}

pub fn get_all_automations(db_path: &PathBuf) -> Result<Vec<Automation>> {
    let conn = Connection::open(db_path)?;

    // Getting the IDs and names of all automations from the automations table
    let mut stmt = conn.prepare("SELECT id, name FROM automations")?;
    let mut rows = stmt.query([])?;
    
    // Creating a list of automations from the results
    let mut automations = Vec::new();

    while let Some(row) = rows.next()? {
        automations.push(Automation {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }

    Ok(automations)
}

pub fn get_triggers_for(db_path: &PathBuf, automation_id: i64) -> Result<Vec<AutomationTrigger>> {
    let conn = Connection::open(db_path)?;

    // Getting all of the triggers for the given automation ID
    let mut stmt = conn.prepare("SELECT id, type, trigger_details FROM triggers WHERE automation_id = ?1")?;
    let mut rows = stmt.query([automation_id])?;

    let mut triggers = Vec::new();

    while let Some(row) = rows.next()? {
        triggers.push(AutomationTrigger {
            id: row.get(0)?,
            trig_type: row.get(1)?,
            details: row.get(2)?,
        });
    }

    Ok(triggers)
}

pub fn get_automation_actions(db_path: &PathBuf, automation_id: i64) -> Result<Vec<AutomationAction>> {
    let conn = Connection::open(db_path)?;

    // Getting all of the actions for the given automation ID
    let mut stmt = conn.prepare("SELECT id, execution_index, action, action_details FROM actions WHERE automation_id = ?1 ORDER BY execution_index ASC")?;
    let mut rows = stmt.query([automation_id])?;

    let mut actions = Vec::new();

    while let Some(row) = rows.next()? {
        actions.push(AutomationAction {
            id: row.get(0)?,
            action_type: row.get(2)?,
            details: row.get(3)?,
        });
    }

    Ok(actions)
}
