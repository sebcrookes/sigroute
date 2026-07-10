use gtk4::{Image, Label, LinkButton, prelude::{ButtonExt, ListBoxRowExt}};
use libadwaita::{ActionRow, Dialog, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{ActionRowExt, AdwDialogExt, PreferencesGroupExt, PreferencesPageExt}};

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
    let vcs_row: ActionRow = ActionRow::builder()
        .title("Source Code")
        .build();

    let vcs_link = LinkButton::new("https://github.com/sebcrookes/sigroute");
    vcs_link.set_label("Sigroute GitHub");
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

    page.add(&settings);

    toolbar_view.set_content(Some(&page));
    menu.set_child(Some(&toolbar_view));

    menu
}