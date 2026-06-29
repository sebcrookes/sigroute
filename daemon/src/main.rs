use sigroute_common::Automation;
use zbus::blocking::connection;
use zbus::interface;

mod db;

struct AutomationAPI;

#[interface(name = "uk.co.sebcrookes.Sigroute")]
impl AutomationAPI {
    fn get_version(&self) -> String {
        return env!("CARGO_PKG_VERSION").to_string();
    }
    
    // fn get_automations(&self) -> Vec<Automation> {
    //     return Vec::new();
    // }

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

    if result.is_err() {
        println!("Error: could not initialise sigroute database.");
        return;
    }

    println!("[Info] - sigrouted running...");
    let _ = run_api();
}

fn run_api() -> zbus::Result<()> {
    let automation_api = AutomationAPI;
    let _connection = connection::Builder::session()?
        .name("uk.co.sebcrookes.Sigroute")?
        .serve_at("/uk/co/sebcrookes/Sigroute", automation_api)?
        .build()?;

    std::thread::park();

    Ok(())
}
