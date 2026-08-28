//! This module provides functionality to allow for times
//! to be selected as options for a FieldMenu. The time
//! is selected in 24-hour format, from 00:00 to 23:59.

use std::collections::HashMap;

use chrono::{Local, Timelike};
use gtk4::{Button, SpinButton, prelude::WidgetExt};
use libadwaita::{Dialog, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{AdwDialogExt, PreferencesGroupExt, PreferencesPageExt}};
use serde_json::json;

use crate::automation::pickers::misc::create_spinbtn_row;

use super::option_picker::OptionPicker;

/// UI component to allow for the selection of a time
/// using a libadwaita Dialog and multiple SpinButtons.
pub struct TimePicker {
    dialog: Dialog,
    hour_picker: SpinButton,
    minute_picker: SpinButton,
    second_picker: SpinButton,
    submit_btn: Button,
}

impl TimePicker {
    /// Constructs a new TimePicker allowing for the user to
    /// select a time - takes in JSON to allow for the pre-
    /// population of the fields. See get_json for the
    /// required format of the JSON.
    pub fn new(json: String) -> Self {
        let picker = Dialog::builder()
            .title("Time Picker")
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

        /* Creating the time entry group */
        let time_group = PreferencesGroup::builder()
            .title("Select Time")
            .build();

        // Getting the current time
        let local_time = Local::now();

        // Setting the default values (either from current time or, if provided, json)
        let jsonified: HashMap<String, i64> = serde_json::from_str(&json).unwrap_or_default();

        let default_hour = match jsonified.get("hour") {
            Some(h) => *h as f64,
            None => local_time.hour() as f64,
        };

        let default_minute = match jsonified.get("minute") {
            Some(m) => *m as f64,
            None => local_time.minute() as f64,
        };

        let default_second = match jsonified.get("second") {
            Some(s) => *s as f64,
            None => local_time.second() as f64,
        };

        /* Adding each of the rows for the time */

        let (row, hour_picker) = create_spinbtn_row("Hour", Some("Select the hour"), 0, 23);
        hour_picker.set_value(default_hour);
        time_group.add(&row);

        let (row, minute_picker) = create_spinbtn_row("Minute", Some("Select the minute"), 0, 59);
        minute_picker.set_value(default_minute);
        time_group.add(&row);

        let (row, second_picker) = create_spinbtn_row("Second", Some("Select the second"), 0, 59);
        second_picker.set_value(default_second);
        time_group.add(&row);

        // Creating the "Submit" button
        let submit_group = PreferencesGroup::new();

        let submit_btn = Button::builder()
            .label("Submit")
            .build();

        submit_btn.add_css_class("success");

        submit_group.add(&submit_btn);

        /* Constructing the page and displaying the dialog to the user */

        page.add(&time_group);
        page.add(&submit_group);

        toolbar_view.set_content(Some(&page));

        Self {
            dialog: picker,
            hour_picker: hour_picker,
            minute_picker: minute_picker,
            second_picker: second_picker,
            submit_btn: submit_btn,
        }
    }
}

impl OptionPicker for TimePicker {
    fn is_now_completed(&self) -> bool {
        return true;
    }

    /// Gets the current state of the picker in JSON, and returns this as
    /// a string.
    /// 
    /// The format for TimePicker is:
    /// {
    ///     "hour": integer,
    ///     "minute": integer,
    ///     "second": integer
    /// }
    fn get_json(&self) -> String {
        let jsonified = json!({
            "hour": self.hour_picker.value_as_int(),
            "minute": self.minute_picker.value_as_int(),
            "second": self.second_picker.value_as_int()
        });

        return jsonified.to_string();
    }

    fn get_submit_button(&self) -> Button {
        return self.submit_btn.clone();
    }

    fn get_summary_text(&self) -> String {
        return format!(
            "{:02}:{:02}:{:02}",
            self.hour_picker.value_as_int(),
            self.minute_picker.value_as_int(),
            self.second_picker.value_as_int()
        );
    }

    fn display(&self, parent: &Dialog) {
        self.dialog.present(Some(parent));
    }

    fn close(&self) {
        self.dialog.close();
    }
}
