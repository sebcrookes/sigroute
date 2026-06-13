use gtk4::*;
use gtk4::prelude::*;

use tokio;

mod api;

#[tokio::main]
async fn main() -> zbus::Result<()> {

    /* Requesting the version number of the daemon (sigrouted) */

    let conn = api::api_open_connection().await?;
    let s = api::api_get_version(conn).await?;

    println!("{}", s);

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
