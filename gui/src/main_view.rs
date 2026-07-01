use gtk4::prelude::GtkWindowExt;
use libadwaita::prelude::AdwApplicationWindowExt;

use crate::sidebar::view::{SidebarView};
use crate::automation::view::{AutomationView};

pub struct MainView {
    pub _window: libadwaita::ApplicationWindow,
    pub _sidebar_view: SidebarView,
    pub _automation_view: AutomationView,
}

impl MainView {
    pub fn new(app: &libadwaita::Application) -> Self {
    
        let window = libadwaita::ApplicationWindow::builder()
            .application(app)
            .title("Sigroute")
            .default_width(420)
            .default_height(320)
            .build();

        let sidebar_view = SidebarView::new();
        let automation_view = AutomationView::new();

        let split_view = libadwaita::NavigationSplitView::builder()
            .sidebar(&sidebar_view.root)
            .content(&automation_view.root)
            .build();

        window.set_content(Some(&split_view));

        window.present();

        Self {
            _window: window,
            _sidebar_view: sidebar_view,
            _automation_view: automation_view,
        }
    }
}
