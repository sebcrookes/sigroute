use std::path::PathBuf;

use sigroute_common::Automation;
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
    
    fn get_automations(&self) -> Vec<Automation> {
        let result = db::get_all_automations(&self.db_path);

        match result {
            Ok(automations) => automations,
            Err(_) => Vec::new(),
        }
    }

    // fn get_automation(&self, index: u64) -> sigroute_common::Automation {
    //     let automation = Automation {
    //         trigger: TimeBased(1),
    //         actions: Vec::new(),
    //     };

    //     return automation;
    // }
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
