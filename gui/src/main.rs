use gtk4::*;
use gtk4::prelude::*;

fn main() {
    let app = Application::builder()
        .application_id("uk.co.sebcrookes.sigroute")
        .build();
    
    app.connect_activate(build_ui);
                
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Sigroute GUI")
        .build();
    
    window.present();
}
