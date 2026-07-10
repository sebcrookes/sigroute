use std::path::PathBuf;

use sigroute_common::APIError::DBAccessError;
use sigroute_common::{APIError, Automation, AutomationAction, AutomationTrigger};
use zbus::blocking::connection;
use zbus::interface;

mod db;

struct AutomationAPI {
    db_path: PathBuf,
}

impl AutomationAPI {
    fn new(db_path: PathBuf) -> Self {
        Self {
            db_path,
        }
    }
}

#[interface(name = "uk.co.sebcrookes.Sigroute")]
impl AutomationAPI {
    fn get_version(&self) -> String {
        return env!("CARGO_PKG_VERSION").to_string();
    }
    
    fn get_automations(&self) -> Result<Vec<Automation>, APIError> {
        let result = db::get_all_automations(&self.db_path);

        match result {
            Ok(automations) => Ok(automations),
            Err(_) => Err(DBAccessError),
        }
    }

    fn get_automation_triggers(&self, automation_id: i64) -> Result<Vec<AutomationTrigger>, APIError> {
        let result = db::get_triggers_for(&self.db_path, automation_id);

        match result {
            Ok(triggers) => Ok(triggers),
            Err(_) => Err(DBAccessError),
        }
    }

    fn get_automation_actions(&self, automation_id: i64) -> Result<Vec<AutomationAction>, APIError> {
        let result = db::get_automation_actions(&self.db_path, automation_id);

        match result {
            Ok(actions) => Ok(actions),
            Err(_) => Err(DBAccessError),
        }
    }

    fn add_automation(&self, automation_name: String) -> Result<i64, APIError> {
        let result = db::add_automation(&self.db_path, automation_name);

        match result {
            Ok(automation_id) => Ok(automation_id),
            Err(_) => Err(DBAccessError),
        }
    }
}

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
