use async_channel::Sender;
use gtk4::{Button, StringList, glib, prelude::{ButtonExt, WidgetExt}};
use libadwaita::{ComboRow, Dialog, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{AdwDialogExt, ComboRowExt, PreferencesGroupExt, PreferencesPageExt}};
use sigroute_common::{TRIGGER_MAX, ACTION_MAX, trigger_to_name, action_to_name};

use crate::message::UIEvent::{self, AddedTrigger};

pub fn create_dialog(sender: &Sender<UIEvent>) -> Dialog {
    let menu = Dialog::builder()
        .title("Add Action")
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
        .title("Action Details")
        .build();

    let actions = ComboRow::builder()
        .title("Action")
        .subtitle("What action to perform")
        .build();
    
    let model = StringList::new(&[]);

    for i in 1..=ACTION_MAX {
        model.append(&action_to_name(i));
    }

    actions.set_model(Some(&model));

    // Creating the "Add" (submit) button
    let submit_group = PreferencesGroup::new();

    let add_btn = Button::builder()
        .label("Add")
        .build();

    add_btn.add_css_class("success");

    let menu_copy = menu.clone();
    let s = sender.clone();
    let actions_copy = actions.clone();
    add_btn.connect_clicked(move |_| {
        // Close the menu, and trigger an event to notify the controller of the new action
        menu_copy.close();

        let s = s.clone();
        let actions_copy = actions_copy.clone();
        glib::spawn_future_local(async move {

        });
    });

    submit_group.add(&add_btn);

    group.add(&actions);
    page.add(&group);
    page.add(&submit_group);
    
    toolbar_view.set_content(Some(&page));
    
    menu
}