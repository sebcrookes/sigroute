use std::{cell::RefCell, sync::Arc};

use async_channel::Sender;
use gtk4::{glib, prelude::{ButtonExt, ListBoxRowExt, WidgetExt}};

use crate::message::Message::{self, AddedAutomation, ChangedAutomation};

pub struct SidebarView {
    pub root: libadwaita::NavigationPage,
    pub list: gtk4::ListBox,
    pub list_ids: Arc<RefCell<Vec<i64>>>,
}

impl SidebarView {
    pub fn new(sender: &Sender<Message>) -> Self {
        let sidebar_list = gtk4::ListBox::new();

        // Adding libadwaita styling to the sidebar
        sidebar_list.add_css_class("navigation-sidebar");

        // Making the sidebar scrollable
        let scrollable_sidebar_list = gtk4::ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vexpand(true)
            .kinetic_scrolling(true)
            .overlay_scrolling(true)
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

        // Registering the handler for the "add automation" button
        let s = sender.clone();
        add_automation_button.connect_clicked(move |_| {
            let s = s.clone();
            glib::spawn_future_local(async move {
                s.send(AddedAutomation).await.unwrap();
            });
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

        let this = Self {
            root: sidebar,
            list: sidebar_list,
            list_ids: Arc::new(RefCell::new(Vec::new())),
        };

        // Setting up the callback for when the row is changed
        let list_id_clone = Arc::clone(&this.list_ids);
        let s = sender.clone();
        this.list.connect_row_activated(move |_, row: &gtk4::ListBoxRow| {
            let s = s.clone();
            let id = list_id_clone.borrow()[row.index() as usize];

            glib::spawn_future_local(async move {
                s.send(ChangedAutomation(id)).await.unwrap();
            });
        });

        this
    }

    pub fn get_current_index(&self) -> i64 {
        if self.list_ids.borrow().is_empty() {
            -1
        } else {
            self.list.selected_row().map_or(-1, |row| row.index().into())
        }
    }

    pub fn select_by_index(&self, index: i64) {
        if let Some(row) = self.list.row_at_index(index.try_into().unwrap()) {
            self.list.select_row(Some(&row));
        }
    }

    pub fn get_current_id(&self) -> i64 {
        let current_index = self.get_current_index();
        if current_index == -1 {
            -1
        } else {
            self.list_ids.borrow()[current_index as usize]
        }
    }

    pub fn clear_automations(&mut self) {
        self.list_ids.borrow_mut().clear();
        while let Some(row) = self.list.first_child() {
            self.list.remove(&row);
        }
    }

    pub fn add_automation(&mut self, name: String, id: i64) {
        self.list_ids.borrow_mut().push(id);
        self.list.append(&construct_sidebar_item(&name));
    }
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
