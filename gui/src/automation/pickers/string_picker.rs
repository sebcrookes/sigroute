use std::collections::HashMap;

use gtk4::{Button, glib, prelude::{EditableExt, WidgetExt}};
use libadwaita::{ApplicationWindow, Dialog, EntryRow, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{AdwDialogExt, EntryRowExt, PreferencesGroupExt, PreferencesPageExt}};
use serde_json::json;

use super::option_picker::OptionPicker;

pub struct StringPicker {
    dialog: Dialog,
    entry_row: EntryRow,
    submit_btn: Button,
}

impl StringPicker {
    pub fn new(window: &ApplicationWindow, should_display: bool, json: String) -> Self {
        let picker = Dialog::builder()
            .title("String Picker")
            .content_width(480)
            .build();

        /* Creating the header, which will provide a close button and the title */

        let header = HeaderBar::builder()
            .build();

        let toolbar_view = ToolbarView::new();
        
        toolbar_view.add_top_bar(&header);
        toolbar_view.set_top_bar_style(libadwaita::ToolbarStyle::Flat);

        picker.set_child(Some(&toolbar_view));

        let page = PreferencesPage::new();

        /* Creating the string entry group */
        let string_group = PreferencesGroup::builder()
            .title("Enter String")
            .build();

        let string_entry_row = EntryRow::builder()
            .title("String")
            .show_apply_button(true)
            .build();
        string_entry_row.connect_apply(glib::clone!(#[weak] window, move |_| {
            gtk4::prelude::GtkWindowExt::set_focus(&window, None::<&gtk4::Widget>);
        }));

        // Setting the default value from the JSON, if provided
        let jsonified: HashMap<String, String> = serde_json::from_str(&json).unwrap_or_default();

        let default_string = match jsonified.get("string") {
            Some(s) => s.clone(),
            None => "".to_string(),
        };

        string_entry_row.set_text(&default_string);

        string_group.add(&string_entry_row);

        // Creating the "Submit" button
        let submit_group = PreferencesGroup::new();

        let submit_btn = Button::builder()
            .label("Submit")
            .build();

        submit_btn.add_css_class("success");

        submit_group.add(&submit_btn);

        /* Constructing the page and displaying the dialog to the user */

        page.add(&string_group);
        page.add(&submit_group);

        toolbar_view.set_content(Some(&page));

        if should_display {
            picker.present(Some(window));
        }

        Self {
            dialog: picker,
            entry_row: string_entry_row,
            submit_btn: submit_btn,
        }
    }
}

impl OptionPicker for StringPicker {
    fn is_now_completed(&self) -> bool {
        return true;
    }

    /// Gets the current state of the picker in JSON, and returns this as
    /// a string.
    /// 
    /// The format for StringPicker is:
    /// {
    ///     "string": string
    /// }
    fn get_json(&self) -> String {
        let jsonified = json!({
            "string": self.entry_row.text().to_string()
        });

        return jsonified.to_string();
    }

    fn get_submit_button(&self) -> Button {
        return self.submit_btn.clone();
    }

    fn get_summary_text(&self) -> String {
        return format!(
            "'{}'",
            self.entry_row.text().to_string()
        );
    }

    fn close(&self) {
        self.dialog.close();
    }
}
