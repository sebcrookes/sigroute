use gtk4::prelude::*;

use tokio;

mod api;
mod sidebar;
mod automation;
mod main_view;

#[tokio::main]
async fn main() -> zbus::Result<()> {

    /* Requesting the version number of the daemon (sigrouted) */

    let conn = api::open_connection().await?;

    let daemon_version = api::get_version(&conn).await?;
    let gui_version = env!("CARGO_PKG_VERSION");

    if gui_version != daemon_version {
        println!("Error - mismatched GUI and daemon versions! sigroute-gui V{}, sigrouted V{}", gui_version, daemon_version)
    }

    println!("{}", daemon_version);

    /* Requesting the list of all automations */

    let automations = api::get_automations(&conn).await?;

    /* Initialising the GUI */

    let app = libadwaita::Application::builder()
        .application_id("uk.co.sebcrookes.SigrouteGUI")
        .build();
    
    app.connect_activate(|app| {
        let _view = main_view::MainView::new(app);
        
    });

    app.run();

    Ok(())
}

//     /* Registering callbacks for click events */

//     // Changing automation in the sidebar
//     let sidebar_changed = move | _list_box: &ListBox, row: &ListBoxRow | {
//         if let Some(widget) = row.child() {
//             if let Some(label) = widget.downcast_ref::<gtk4::Label>() {
//                 let content_header_label = gtk4::Label::builder()
//                     .use_markup(true)
//                     .label(format!("<b>{}</b>", label.text()))
//                     .margin_start(20)
//                     .margin_end(20)
//                     .build();

//                 content_header.set_title_widget(Some(&content_header_label));
//             }
//         }
//     };

//     // Initialising the content header's title
//     let first_child = sidebar_list.first_child();

//     match first_child {
//         Some(child) => {
//             if let Some(row) = child.downcast_ref::<ListBoxRow>() {
//                 sidebar_changed(&sidebar_list, &row);
//             }
//         }
//         None => {}
//     }

//     sidebar_list.connect_row_activated(sidebar_changed);
