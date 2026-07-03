use gtk4::{glib, prelude::*};

use crate::{automation::controller::AutomationController, main_controller::MainController, main_view::MainViewConstructor, message::Message, sidebar::controller::SidebarController};

mod api;
mod sidebar;
mod automation;
mod main_view;
mod main_controller;
mod app_model;
mod message;

fn main() -> zbus::Result<()> {

    /* Initialising the application */

    let app = libadwaita::Application::builder()
        .application_id("uk.co.sebcrookes.SigrouteGUI")
        .build();
    
    app.connect_activate(move |app| {
        let (sender, receiver) = async_channel::unbounded::<Message>();

        // Constructing the application's view
        let view: MainViewConstructor = MainViewConstructor::new(app, &sender);
        
        // Constructing the controllers for each of the components
        let sidebar_controller = SidebarController::new(view.sidebar_view);
        let automation_controller = AutomationController::new(view.automation_view);

        glib::spawn_future_local(async move {

            // Initialising the API connection for the controller
            let conn_result = api::open_connection().await;
            let conn = match conn_result {
                Ok(conn) => conn,
                Err(_) => {
                    println!("Error - could not connect to the daemon");
                    std::process::exit(1);
                },
            };

            // Checking the version of the daemon matches the version of the GUI
            let daemon_version_result = api::get_version(&conn).await;
            let daemon_version = match daemon_version_result {
                Ok(version) => version,
                Err(_) => {
                    println!("Error - failed to query the daemon version (maybe the daemon is not running?)");
                    std::process::exit(1);
                }
            };

            let gui_version = env!("CARGO_PKG_VERSION");
            if gui_version != daemon_version {
                println!("Error - mismatched GUI and daemon versions! sigroute-gui V{}, sigrouted V{}", gui_version, daemon_version);
                std::process::exit(1);
            }

            // Constructing the main controller (transferring ownership to it)
            let mut main_controller = MainController::new(conn, sidebar_controller, automation_controller).await;

            // Entering the main message loop
            while let Ok(message) = receiver.recv().await {
                main_controller.handle(message).await;
            }
        });
    });

    app.run();

    Ok(())
}
