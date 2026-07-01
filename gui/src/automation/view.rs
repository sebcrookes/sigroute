use libadwaita::prelude::{PreferencesGroupExt, PreferencesPageExt};

// fn construct_sidebar_item(title: &str) -> gtk4::ListBoxRow {
//     let row = gtk4::ListBoxRow::new();
//     row.set_halign(gtk4::Align::Fill);

//     let label = gtk4::Label::new(Some(title));
//     label.set_hexpand(true);
//     label.set_margin_start(20);
//     label.set_margin_end(20);

//     row.set_child(Some(&label));

//     row
// }

pub struct AutomationView {
    pub root: libadwaita::NavigationPage,
}

impl AutomationView {
    pub fn new() -> Self {
        
        let content_header = libadwaita::HeaderBar::builder()
            .title_widget(&gtk4::Label::builder().use_markup(true).label("<b></b>").halign(gtk4::Align::Start).margin_end(20).margin_start(20).build())
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

        // automation_title_entry.connect_apply(glib::clone!(#[weak] window, move |_| {
        //     gtk4::prelude::GtkWindowExt::set_focus(&window, None::<&gtk4::Widget>);
        // }));

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

        Self {
            root: content,
        }
    }
}
