use std::collections::HashMap;

use chrono::{Local, Timelike};
use gtk4::{Button, SpinButton, prelude::{WidgetExt}};
use libadwaita::{ActionRow, ApplicationWindow, Dialog, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{ActionRowExt, AdwDialogExt, PreferencesGroupExt, PreferencesPageExt}};
use serde_json::json;

use crate::automation::option_picker::{OptionPicker};

pub struct TimePicker {
    dialog: Dialog,
    hour_picker: SpinButton,
    minute_picker: SpinButton,
    second_picker: SpinButton,
    submit_btn: Button,
}

impl TimePicker {
    pub fn new(window: &ApplicationWindow, should_display: bool, json: String) -> Self {
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

        let (row, hour_picker) = create_row("Hour", "Select the hour", 0, 23);
        hour_picker.set_value(default_hour);
        time_group.add(&row);

        let (row, minute_picker) = create_row("Minute", "Select the minute", 0, 59);
        minute_picker.set_value(default_minute);
        time_group.add(&row);

        let (row, second_picker) = create_row("Second", "Select the second", 0, 59);
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

        if should_display {
            picker.present(Some(window));
        }

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

    fn close(&self) {
        self.dialog.close();
    }
}

fn create_row(title: &str, subtitle: &str, min: i64, max: i64) -> (ActionRow, SpinButton) {
    let row = ActionRow::builder()
        .title(title)
        .subtitle(subtitle)
        .build();

    let picker = create_spin_button(min, max);
    row.add_suffix(&picker);

    return (row, picker);
}

fn create_spin_button(min: i64, max: i64) -> SpinButton {
    let spin_button = SpinButton::with_range(min as f64, max as f64, 1.0);
    spin_button.set_numeric(true);
    spin_button.set_digits(0);
    spin_button.set_snap_to_ticks(true);
    spin_button.set_margin_top(8);
    spin_button.set_margin_bottom(8);

    return spin_button;
}
