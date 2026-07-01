use gtk4::gdk::Display;
use gtk4::*;
use gtk4::prelude::*;

use libadwaita::prelude::{AdwApplicationWindowExt, EntryRowExt, PreferencesGroupExt, PreferencesPageExt};
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

    let app = libadwaita::Application::builder()
        .application_id("uk.co.sebcrookes.SigrouteGUI")
        .build();
    
    app.connect_activate(move |app| {
        build_ui(app, &automations);
    });

    app.run();

    Ok(())
}

fn build_ui(app: &libadwaita::Application, automations: &Vec<Automation>) {

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
    row.set_halign(gtk4::Align::Fill);

    let label = gtk4::Label::new(Some(title));
    label.set_hexpand(true);
    label.set_margin_start(20);
    label.set_margin_end(20);

    row.set_child(Some(&label));

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

    let title_label = gtk4::Label::builder()
        .use_markup(true)
        .label("<b>Sigroute</b>")
        .build();

    let sidebar_header = libadwaita::HeaderBar::builder()
        .title_widget(&title_label)
        .build();

    // Adding the "add automation" button to the header bar
    let add_automation_button = gtk4::Button::builder()
        .icon_name("list-add-symbolic")
        .build();
    sidebar_header.pack_start(&add_automation_button);

    add_automation_button.connect_clicked(|_| {
        println!("CLICKED");
    });

    // Adding the menu button to the header bar
    let menu_button = gtk4::Button::builder()
        .icon_name("open-menu-symbolic")
        .build();
    sidebar_header.pack_end(&menu_button);

    let sidebar_toolbar = libadwaita::ToolbarView::builder()
        .content(&scrollable_sidebar_list)
        .build();

    sidebar_toolbar.add_top_bar(&sidebar_header);  
    sidebar_toolbar.set_top_bar_style(libadwaita::ToolbarStyle::Flat);  

    let sidebar = libadwaita::NavigationPage::builder()
        .child(&sidebar_toolbar)
        .title("Automations List")
        .build();


    /* Constructing the content pane */

    let content_header = libadwaita::HeaderBar::builder()
        .title_widget(&gtk4::Label::builder().use_markup(true).label("<b></b>").halign(Align::Start).margin_end(20).margin_start(20).build())
        .build();

    let automation_info = libadwaita::PreferencesPage::builder()
        .build();

    /* Automation details */

    let automation_details_group = libadwaita::PreferencesGroup::builder()
        .title("Details")
        .build();

    let automation_title_entry = libadwaita::EntryRow::builder()
        .title("Name")
        .show_apply_button(true)
        .build();

    automation_title_entry.connect_apply(glib::clone!(#[weak] window, move |_| {
        gtk4::prelude::GtkWindowExt::set_focus(&window, None::<&gtk4::Widget>);
    }));

    let automation_title = libadwaita::PreferencesRow::builder()
        .title("Name")
        .child(&automation_title_entry)
        .build();
    automation_details_group.add(&automation_title);

    automation_info.add(&automation_details_group);

    let automation_triggers_group = libadwaita::PreferencesGroup::builder()
        .title("Triggers")
        .build();

    automation_info.add(&automation_triggers_group);

    let automation_actions_group = libadwaita::PreferencesGroup::builder()
        .title("Actions")
        .build();

    automation_info.add(&automation_actions_group);
    
    let content_toolbar = libadwaita::ToolbarView::builder()
        .content(&automation_info)
        .build();
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



    /* Registering callbacks for click events */

    // Changing automation in the sidebar
    let sidebar_changed = move | _list_box: &ListBox, row: &ListBoxRow | {
        if let Some(widget) = row.child() {
            if let Some(label) = widget.downcast_ref::<gtk4::Label>() {
                let content_header_label = gtk4::Label::builder()
                    .use_markup(true)
                    .label(format!("<b>{}</b>", label.text()))
                    .margin_start(20)
                    .margin_end(20)
                    .build();

                content_header.set_title_widget(Some(&content_header_label));
            }
        }
    };

    // Initialising the content header's title
    let first_child = sidebar_list.first_child();

    match first_child {
        Some(child) => {
            if let Some(row) = child.downcast_ref::<ListBoxRow>() {
                sidebar_changed(&sidebar_list, &row);
            }
        }
        None => {}
    }

    sidebar_list.connect_row_activated(sidebar_changed);
    
}
