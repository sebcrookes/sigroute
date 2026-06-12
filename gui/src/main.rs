use gtk4::*;
use gtk4::prelude::*;

use zbus::Connection;
use zbus::proxy;

use tokio;

#[proxy(
    interface = "uk.co.sebcrookes.Sigroute",
    default_service = "uk.co.sebcrookes.Sigroute",
    default_path = "/uk/co/sebcrookes/Sigroute",
    gen_blocking = true
)]
trait AutomationAPI {
    fn list_all(&self) -> zbus::Result<String>;
}

async fn list_automations() -> zbus::Result<String> {
    let connection = Connection::session().await?;
    let proxy = AutomationAPIProxy::new(&connection).await?;

    let reply = proxy.list_all().await;

    return reply;
}

fn main() {

    /* Requesting the list of automations from the daemon (sigrouted) */
    let rt = tokio::runtime::Runtime::new().unwrap();

    let result = rt.block_on(list_automations());

    match result {
        Ok(retv) => println!("{retv}"),
        Err(_e) => println!("Error!"),
    };

    /* Initialising the GUI */

    let app = Application::builder()
        .application_id("uk.co.sebcrookes.SigrouteGUI")
        .build();
    
    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Sigroute")
        .default_width(420)
        .default_height(320)
        .build();
    
    window.present();
}
