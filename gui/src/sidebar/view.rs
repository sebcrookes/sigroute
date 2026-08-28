//! This module provides the view displaying all of the automations
//! in the system. These can be selected, and can be deactivated.
//! Additionally, this module provides the ability to open an
//! about/settings page (defined in a different module), which
//! displays information about the application and various options
//! to the user.

use async_channel::Sender;
use gtk4::{ListBoxRow, glib, pango::EllipsizeMode, prelude::{ButtonExt, ListBoxRowExt, WidgetExt}};
use libadwaita::{ApplicationWindow, NavigationPage};

use crate::{app_model::AppModel, message::{ModelUpdate::{self, AutomationListUpdate}, UIEvent::{self, AddedAutomation, ChangedAutomation}}, sidebar::menu::SettingsMenu};

/// UI component allowing for the display of a list of automations,
/// along with a settings page and an "add automation" button.
pub struct SidebarView {
    pub root: NavigationPage,
    pub list: gtk4::ListBox,
    pub list_rows: Vec<ListBoxRow>
}

impl SidebarView {
    /// Constructor for the sidebar, which requires the UI event sender
    /// and the application's window.
    pub fn new(sender: &Sender<UIEvent>, window: &ApplicationWindow) -> Self {
        let sidebar_list = gtk4::ListBox::new();

        // Adding libadwaita styling to the sidebar
        sidebar_list.add_css_class("navigation-sidebar");

        // Making theabout sidebar scrollable
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
            .focus_on_click(false)
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
            .focus_on_click(false)
            .build();
        sidebar_header.pack_end(&menu_button);

        // Menu dialog

        let window_clone = window.clone();
        menu_button.connect_clicked(move |_| {
            let menu = SettingsMenu::new();
            menu.display(&window_clone);
        });

        // Toolbar
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
            list_rows: Vec::new(),
        };

        // Setting up the callback for when the row is changed
        let s = sender.clone();
        this.list.connect_row_activated(move |_, row: &gtk4::ListBoxRow| {
            let s = s.clone();

            let index: i64 = row.index().into();

            glib::spawn_future_local(async move {
                s.send(ChangedAutomation(index)).await.unwrap();
            });
        });

        this
    }

    /// Returns the root of the view (NavigationPage).
    pub fn get_root(&self) -> NavigationPage {
        return self.root.clone();
    }

    /// Performs actions based on a notification sent to the
    /// view by the controller.
    pub async fn handle_model_update(&mut self, model: &mut AppModel, message: ModelUpdate) {
        match message {
            AutomationListUpdate => {
                self.render(model).await;
            }
            _ => {}
        }
    }

    /// Redraws the automation list in the sidebar, based on
    /// the data in the provided model.
    pub async fn render(&mut self, model: &AppModel) {
        self.clear_automations();

        for automation in &model.automations {
            self.add_automation(automation.name.clone(), automation.active);
        }

        self.select_by_index(model.get_current_automation_index());
    }

    /// Selects the row at the provided index.
    pub fn select_by_index(&self, index: i64) {
        if let Some(row) = self.list.row_at_index(index.try_into().unwrap()) {
            self.list.select_row(Some(&row));
        }
    }

    /// Removes all of the registered automations from the
    /// sidebar.
    pub fn clear_automations(&mut self) {
        for row in &self.list_rows {
            self.list.remove(row);
        }
        self.list_rows.clear();
    }

    /// Adds an automation to the sidebar with the given name
    /// and activity status.
    pub fn add_automation(&mut self, name: String, active: bool) {
        let row = construct_sidebar_item(&name, active);
        self.list.append(&row);
        self.list_rows.push(row);
    }
}

/// Constructs a sidebar item (automation) from its title
/// and its activity status.
fn construct_sidebar_item(title: &str, active: bool) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.set_halign(gtk4::Align::Fill);

    let label = gtk4::Label::new(Some(title));
    label.set_hexpand(true);
    label.set_margin_start(20);
    label.set_margin_end(20);
    label.set_ellipsize(EllipsizeMode::End);
    label.set_max_width_chars(25);
    label.set_width_chars(20);
    label.set_lines(1);

    if !active {
        label.set_markup(&format!("<span foreground=\"gray\">{title}</span>"));
    }

    row.set_child(Some(&label));
    row
}
