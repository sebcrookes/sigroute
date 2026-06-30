use gtk4::*;
use gtk4::prelude::*;

use libadwaita::prelude::AdwApplicationWindowExt;
use sigroute_common::Automation;
use tokio;

mod api;

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

    let app = Application::builder()
        .application_id("uk.co.sebcrookes.SigrouteGUI")
        .build();
    
    app.connect_activate(move |app| {
        build_ui(app, &automations);
    });

    app.run();

    Ok(())
}

fn build_ui(app: &Application, automations: &Vec<Automation>) {
    let window = libadwaita::ApplicationWindow::builder()
        .application(app)
        .title("Sigroute")
        .default_width(420)
        .default_height(320)
        .build();

    construct_contents(&window, automations);

    window.present();
}

fn construct_sidebar_item(title: &str) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();

    let item_box = Box::new(Orientation::Horizontal, 10);
    item_box.set_margin_start(15);
    item_box.set_margin_end(15);
    item_box.set_margin_top(10);
    item_box.set_margin_bottom(10);

    let label = gtk4::Label::new(Some(title));
    label.set_hexpand(true);

    item_box.append(&label);

    row.set_child(Some(&item_box));

    row
}

fn construct_contents(window: &libadwaita::ApplicationWindow, automations: &Vec<Automation>) {

    /* Constructing the sidebar */

    let sidebar_list = gtk4::ListBox::new();

    // Adding libadwaita styling to the sidebar
    sidebar_list.add_css_class("navigation-sidebar");

    // Adding all of the automations to the sidebar
    for automation in automations {
        sidebar_list.append(&construct_sidebar_item(&automation.name));
    }

    // TODO: Add a "no automations" message

    // Making the sidebar scrollable
    let scrollable_sidebar_list = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vexpand(true)
        .kinetic_scrolling(true)
        .overlay_scrolling(true)
        .propagate_natural_width(true)
        .propagate_natural_height(true)
        .has_frame(false)
        .child(&sidebar_list)
        .build();

    let sidebar_header = libadwaita::HeaderBar::builder()
        .title_widget(&gtk4::Label::new(Some("Sigroute GUI")))
        .build();

    let sidebar_toolbar = libadwaita::ToolbarView::builder()
        .content(&scrollable_sidebar_list)
        .build();

    sidebar_toolbar.add_top_bar(&sidebar_header);    

    let sidebar = libadwaita::NavigationPage::builder()
        .child(&sidebar_toolbar)
        .title("Automations List")
        .build();


    /* Constructing the content pane */

    let content_header = libadwaita::HeaderBar::builder()
        .show_title(false)
        .build();

    let content_toolbar = libadwaita::ToolbarView::new();
    content_toolbar.add_top_bar(&content_header);

    let content = libadwaita::NavigationPage::builder()
        .child(&content_toolbar)
        .title("Automation")
        .build();


    /* Constructing the split view */

    let split_view = libadwaita::NavigationSplitView::builder()
        .sidebar(&sidebar)
        .content(&content)
        .build();

    window.set_content(Some(&split_view));
   
}
