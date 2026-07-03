use async_channel::Sender;
use gtk4::glib;
use libadwaita::{ApplicationWindow, EntryRow, HeaderBar, NavigationPage, PreferencesGroup, PreferencesPage, PreferencesRow, ToolbarView, prelude::{EntryRowExt, PreferencesGroupExt, PreferencesPageExt}};

use crate::message::Message;

pub struct AutomationView {
    pub root: NavigationPage,
}

impl AutomationView {
    pub fn new(sender: &Sender<Message>, window: &ApplicationWindow) -> Self {
        
        let content_header = HeaderBar::builder()
            .title_widget(&gtk4::Label::builder().use_markup(true).label("<b></b>").halign(gtk4::Align::Start).margin_end(20).margin_start(20).build())
            .build();

        let automation_info = PreferencesPage::builder()
            .build();

        /* Automation details */

        let automation_details_group = PreferencesGroup::builder()
            .title("Details")
            .build();

        let automation_title_entry = EntryRow::builder()
            .title("Name")
            .show_apply_button(true)
            .build();

        automation_title_entry.connect_apply(glib::clone!(#[weak] window, move |_| {
            gtk4::prelude::GtkWindowExt::set_focus(&window, None::<&gtk4::Widget>);
        }));

        let automation_title = PreferencesRow::builder()
            .title("Name")
            .child(&automation_title_entry)
            .build();
        automation_details_group.add(&automation_title);

        automation_info.add(&automation_details_group);

        let automation_triggers_group = PreferencesGroup::builder()
            .title("Triggers")
            .build();

        automation_info.add(&automation_triggers_group);

        let automation_actions_group = PreferencesGroup::builder()
            .title("Actions")
            .build();

        automation_info.add(&automation_actions_group);
        
        let content_toolbar = ToolbarView::builder()
            .content(&automation_info)
            .build();
        content_toolbar.add_top_bar(&content_header);

        let content = NavigationPage::builder()
            .child(&content_toolbar)
            .title("Automation")
            .build();

        Self {
            root: content,
        }
    }
}
