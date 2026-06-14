use gtk4::*;
use gtk4::prelude::*;

use tokio;

mod api;

#[tokio::main]
async fn main() -> zbus::Result<()> {

    /* Requesting the version number of the daemon (sigrouted) */

    let conn = api::open_connection().await?;

    let daemon_version = api::get_version(conn).await?;
    let gui_version = env!("CARGO_PKG_VERSION");

    if gui_version != daemon_version {
        println!("Error - mismatched GUI and daemon versions! sigroute-gui V{}, sigrouted V{}", gui_version, daemon_version)
    }

    println!("{}", daemon_version);

    /* Initialising the GUI */

    let app = Application::builder()
        .application_id("uk.co.sebcrookes.SigrouteGUI")
        .build();
    
    app.connect_activate(build_ui);

    app.run();

    Ok(())
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
