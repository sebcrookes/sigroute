use async_channel::Sender;
use gtk4::prelude::GtkWindowExt;
use libadwaita::prelude::AdwApplicationWindowExt;

use crate::message::Message;
use crate::sidebar::view::{SidebarView};
use crate::automation::view::{AutomationView};

pub struct MainViewConstructor {
    pub sidebar_view: SidebarView,
    pub automation_view: AutomationView,
}

impl MainViewConstructor {
    pub fn new(app: &libadwaita::Application, sender: &Sender<Message>) -> Self {
    
        let window = libadwaita::ApplicationWindow::builder()
            .application(app)
            .title("Sigroute")
            .default_width(420)
            .default_height(320)
            .build();

        let sidebar_view = SidebarView::new(sender);
        let automation_view = AutomationView::new(sender, &window);

        let split_view = libadwaita::NavigationSplitView::builder()
            .sidebar(&sidebar_view.root)
            .content(&automation_view.root)
            .build();

        window.set_content(Some(&split_view));

        window.present();

        Self {
            sidebar_view: sidebar_view,
            automation_view: automation_view,
        }
    }
}
