use zbus::blocking::connection;
use zbus::interface;

struct AutomationAPI;

#[interface(name = "uk.co.sebcrookes.Sigroute")]
impl AutomationAPI {
    fn list_all(&self) -> String {
        return "This is a test string being returned by sigrouted".to_string();
    }
}

fn main() {
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
