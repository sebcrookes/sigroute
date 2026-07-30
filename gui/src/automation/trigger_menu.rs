use async_channel::Sender;
use gtk4::{Button, StringList, glib, prelude::{ButtonExt, WidgetExt}};
use libadwaita::{ComboRow, Dialog, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{AdwDialogExt, ComboRowExt, PreferencesGroupExt, PreferencesPageExt}};
use sigroute_common::{TRIGGER_MAX, trigger_to_name};

use crate::message::UIEvent::{self, AddedTrigger};

pub fn create_dialog(sender: &Sender<UIEvent>) -> Dialog {
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
    
    let model = StringList::new(&[]);

    for i in 1..=TRIGGER_MAX {
        model.append(&trigger_to_name(i));
    }

    triggers.set_model(Some(&model));

    // Creating the "Add" (submit) button
    let submit_group = PreferencesGroup::new();

    let add_btn = Button::builder()
        .label("Add")
        .build();

    add_btn.add_css_class("success");

    let menu_copy = menu.clone();
    let s = sender.clone();
    let triggers_copy = triggers.clone();
    add_btn.connect_clicked(move |_| {
        // Close the menu, and trigger an event to notify the controller of the new trigger
        menu_copy.close();

        let s = s.clone();
        let triggers_copy = triggers_copy.clone();
        glib::spawn_future_local(async move {
            s.send(AddedTrigger((triggers_copy.selected() + 1).into(), "".to_string())).await.unwrap();
        });
    });

    submit_group.add(&add_btn);

    group.add(&triggers);
    page.add(&group);
    page.add(&submit_group);
    
    toolbar_view.set_content(Some(&page));
    
    menu
}
