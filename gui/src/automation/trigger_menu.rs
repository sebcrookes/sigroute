use gtk4::{Button, prelude::{ButtonExt, WidgetExt}};
use libadwaita::{ComboRow, Dialog, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{AdwDialogExt, PreferencesGroupExt, PreferencesPageExt}};

pub fn create_dialog() -> Dialog {
    let menu = Dialog::builder()
        .title("Add Trigger")
        .content_width(480)
        .build();

    // Creating the header, which will provide a close button and the title
    let header = HeaderBar::builder()
        .build();

    let toolbar_view = ToolbarView::new();
    
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_top_bar_style(libadwaita::ToolbarStyle::Flat);

    menu.set_child(Some(&toolbar_view));

    let page = PreferencesPage::new();

    let group = PreferencesGroup::builder()
        .title("Trigger Details")
        .build();

    let triggers = ComboRow::builder()
        .title("Trigger Type")
        .subtitle("What to trigger on")
        .build();

    // Creating the "Add" (submit) button
    let submit_group = PreferencesGroup::new();

    let add_btn = Button::builder()
        .label("Add")
        .build();

    add_btn.add_css_class("success");

    let menu_copy = menu.clone();
    add_btn.connect_clicked(move |_| {
        // Close the menu, and trigger an event to notify the controller of the new trigger
        menu_copy.close();
    });

    submit_group.add(&add_btn);

    group.add(&triggers);
    page.add(&group);
    page.add(&submit_group);
    
    toolbar_view.set_content(Some(&page));
    
    menu
}
