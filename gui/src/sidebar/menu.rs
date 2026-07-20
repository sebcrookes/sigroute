use gtk4::{DropDown, Image, Label, StringList, glib};
use libadwaita::{ActionRow, Dialog, HeaderBar, PreferencesGroup, PreferencesPage, StyleManager, ToolbarView, prelude::{ActionRowExt, AdwDialogExt, PreferencesGroupExt, PreferencesPageExt}};

pub fn create_dialog() -> Dialog {
    // Constructing the dialog (the popup)
    let menu = Dialog::builder()
            .title("Menu")
            .content_width(480)
            .content_height(320)
            .build();

    // Creating the header, which will provide a close button and the "menu" title
    let header = HeaderBar::builder()
        .build();

    let toolbar_view = ToolbarView::new();
    
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_top_bar_style(libadwaita::ToolbarStyle::Flat);  

    // Creating the preferences page which will hold all of the content
    let page = PreferencesPage::builder()
        .margin_start(10)
        .margin_end(10)
        .build();

    // Creating an about/credits menu
    let about: PreferencesGroup = PreferencesGroup::builder()
        .title("About")
        .build();

    // Version
    let version_row = ActionRow::builder()
        .title("Version")
        .build();
    version_row.add_suffix(&Label::new(Some(env!("CARGO_PKG_VERSION"))));

    let version_icon = Image::new();
    version_icon.set_icon_name(Some("software-update-available-symbolic"));
    version_row.add_prefix(&version_icon);

    about.add(&version_row);

    // Link to VCS (source code)
    let vcs_row = ActionRow::builder()
        .title("Source Code")
        .build();

    let vcs_link = Label::new(Some(&"<a href=\"https://github.com/sebcrookes/sigroute\">GitHub</a>"));
    vcs_link.set_use_markup(true);
    vcs_row.add_suffix(&vcs_link);

    let vcs_icon = Image::new();
    vcs_icon.set_icon_name(Some("utilities-terminal-symbolic"));
    vcs_row.add_prefix(&vcs_icon);

    about.add(&vcs_row);

    page.add(&about);

    // Creating the settings menus
    let settings = PreferencesGroup::builder()
        .title("Settings")
        .build();

    // Colour scheme selection
    let theme_row = ActionRow::builder()
        .title("Theme")
        .build();

    let theme_icon = Image::new();
    theme_icon.set_icon_name(Some("preferences-desktop-appearance-symbolic"));
    theme_row.add_prefix(&theme_icon);

    let theme_dropdown = DropDown::builder()
        .name("Theme")
        .margin_top(8)
        .margin_bottom(8)
        .build();

    let theme_options = vec!["Default", "Light", "Dark"];
    let theme_strings = StringList::new(&theme_options);
    theme_dropdown.set_model(Some(&theme_strings));

    theme_dropdown.connect_selected_notify(|dropdown| {
        let index = dropdown.selected();

        let colour_scheme = match index {
            1 => libadwaita::ColorScheme::ForceLight,
            2 => libadwaita::ColorScheme::ForceDark,
            _ => libadwaita::ColorScheme::Default, // Including 0
        };

        glib::idle_add_local(move || {
            let style_manager = StyleManager::default();
            style_manager.set_color_scheme(colour_scheme);
            glib::ControlFlow::Break
        });
    });

    theme_row.add_suffix(&theme_dropdown);

    settings.add(&theme_row);

    page.add(&settings);

    toolbar_view.set_content(Some(&page));
    menu.set_child(Some(&toolbar_view));

    menu
}
