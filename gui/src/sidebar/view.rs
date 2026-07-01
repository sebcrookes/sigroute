use gtk4::prelude::WidgetExt;

pub struct SidebarView {
    pub root: libadwaita::NavigationPage,
    pub _list: gtk4::ListBox,
}

impl SidebarView {
    pub fn new() -> Self {
        let sidebar_list = gtk4::ListBox::new();

        // Adding libadwaita styling to the sidebar
        sidebar_list.add_css_class("navigation-sidebar");

        // // Adding all of the automations to the sidebar
        // for automation in automations {
        //     sidebar_list.append(&construct_sidebar_item(&automation.name));
        // }

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

        Self {
            root: sidebar,
            _list: sidebar_list,
        }
    }
}
