//! This module constructs the main window and views for
//! the application - the sidebar view and the automation view.

use async_channel::Sender;
use gtk4::prelude::GtkWindowExt;
use libadwaita::{ApplicationWindow, NavigationSplitView};
use libadwaita::prelude::AdwApplicationWindowExt;

use crate::message::UIEvent;
use crate::sidebar::view::SidebarView;
use crate::automation::view::AutomationView;

/// A struct which contains the two main views of the
/// application, created by the constructor for this struct
/// 'new'.
pub struct MainViewConstructor {
    pub sidebar_view: SidebarView,
    pub automation_view: AutomationView,
}

impl MainViewConstructor {
    /// Constructor which creates the window for the application
    /// and the two main views, and puts them together into a
    /// NavigationSplitView.
    pub fn new(app: &libadwaita::Application, sender: &Sender<UIEvent>) -> Self {
    
        // Constructing the window for the application
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Sigroute")
            .default_width(840)
            .default_height(520)
            .build();

        // Constructing the two main views in the application
        let sidebar_view = SidebarView::new(sender, &window);
        let automation_view = AutomationView::new(sender, &window);

        // Adding them to a NavigationSplitView
        let split_view = NavigationSplitView::builder()
            .sidebar(&sidebar_view.get_root())
            .content(&automation_view.get_root())
            .build();

        window.set_content(Some(&split_view));

        window.present();

        Self {
            sidebar_view: sidebar_view,
            automation_view: automation_view,
        }
    }
}
