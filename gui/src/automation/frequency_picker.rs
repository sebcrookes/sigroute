use std::collections::HashMap;

use gtk4::{Button, SpinButton, prelude::{EditableExt, WidgetExt}};
use libadwaita::{ActionRow, ApplicationWindow, Dialog, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{ActionRowExt, AdwDialogExt, PreferencesGroupExt, PreferencesPageExt}};
use serde_json::json;

use crate::automation::option_picker::OptionPicker;

pub struct FrequencyPicker {
    dialog: Dialog,
    year_picker: SpinButton,
    month_picker: SpinButton,
    day_picker: SpinButton,
    hour_picker: SpinButton,
    minute_picker: SpinButton,
    second_picker: SpinButton,
    submit_btn: Button,
}

impl FrequencyPicker {
    pub fn new(window: &ApplicationWindow, json: String) -> Self {
        let picker = Dialog::builder()
            .title("Frequency Picker")
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

        /* Creating the time selection group */

        let gregorian_time_group = PreferencesGroup::builder()
            .title("Select Frequency")
            .build();

        // Adding a hint as to how the selector works
        gregorian_time_group.set_description(Some("You can pick a whole number of years, or months, or days, or a combination of hours, minutes and seconds"));
        
        /* Adding each of the rows for the individual units of time */

        let year = create_row("Years", "Number of years", 0, 100000);
        gregorian_time_group.add(&year.0);

        let month = create_row("Months", "Number of months", 0, 100000);
        gregorian_time_group.add(&month.0);

        let day = create_row("Days", "Number of days", 0, 100000);
        gregorian_time_group.add(&day.0);

        let hms_group = PreferencesGroup::new();

        let hour = create_row("Hours", "Number of hours", 0, 100000);
        hms_group.add(&hour.0);

        let minute = create_row("Minutes", "Number of minutes", 0, 100000);
        hms_group.add(&minute.0);

        let second = create_row("Seconds", "Number of seconds", 0, 100000);
        hms_group.add(&second.0);

        /* Creating the logic to disable other fields based on which option (years, minutes, days, hms) is in use */

        create_all_disable_callbacks(vec![year.clone(), month.clone(), day.clone()], vec![hour.clone(), minute.clone(), second.clone()]);

        /* Setting the default values of the rows from the provided JSON */
        
        let jsonified: HashMap<String, i64> = serde_json::from_str(&json).unwrap_or_default();

        if let Some(y) = jsonified.get("year") {
            year.1.set_value(*y as f64);
        } else if let Some(mo) = jsonified.get("month") {
            month.1.set_value(*mo as f64);
        } else if let Some(d) = jsonified.get("day") {
            day.1.set_value(*d as f64);
        } else {
            if let Some(h) = jsonified.get("hour") {
                hour.1.set_value(*h as f64);
            }

            if let Some(m) = jsonified.get("minute") {
                minute.1.set_value(*m as f64);
            }

            if let Some(s) = jsonified.get("second") {
                second.1.set_value(*s as f64);
            }
        }

        // Creating the "Submit" button
        let submit_group = PreferencesGroup::new();

        let submit_btn = Button::builder()
            .label("Submit")
            .build();

        submit_btn.add_css_class("success");

        submit_group.add(&submit_btn);

        /* Constructing the page and displaying the dialog to the user */

        page.add(&gregorian_time_group);
        page.add(&hms_group);
        page.add(&submit_group);

        toolbar_view.set_content(Some(&page));

        picker.present(Some(window));

        Self {
            dialog: picker,
            year_picker: year.1,
            month_picker: month.1,
            day_picker: day.1,
            hour_picker: hour.1,
            minute_picker: minute.1,
            second_picker: second.1,
            submit_btn: submit_btn,
        }
    }
}

impl OptionPicker for FrequencyPicker {
    fn is_now_completed(&self) -> bool {
        /* The picker has not been completed if the values are all zero */
        let pickers = [
                                            &self.year_picker,
                                            &self.month_picker,
                                            &self.day_picker,
                                            &self.hour_picker,
                                            &self.minute_picker,
                                            &self.second_picker
                                        ];

        let mut completed = false;
        for picker in pickers {
            completed |= picker.value_as_int() != 0;
        }

        return completed;
    }

    /// Gets the current state of the picker in JSON, and returns this as
    /// a string.
    /// 
    /// The format for FrequencyPicker is:
    /// {
    ///     "year": integer,
    ///     "month": integer,
    ///     "day": integer,
    /// 
    ///     "hour": integer,
    ///     "minute": integer,
    ///     "second": integer
    /// }
    /// 
    /// Please note that only year, month and day are only to be used on their own,
    /// and not with any of the other fields. For example, year cannot be used with month,
    /// and day cannot be used with hour. If you want the frequency to be 1 day and 2
    /// hours, please just select 26 hours.
    fn get_json(&self) -> String {
        if self.hour_picker.value_as_int() != 0
            || self.minute_picker.value_as_int() != 0
            || self.second_picker.value_as_int() != 0 {
                
            let jsonified_hms = json!({
                "hour": self.hour_picker.value_as_int(),
                "minute": self.minute_picker.value_as_int(),
                "second": self.second_picker.value_as_int()
            });

            return jsonified_hms.to_string();
        } else if self.day_picker.value_as_int() != 0 {
            let jsonified_day = json!({
                "day": self.day_picker.value_as_int()
            });

            return jsonified_day.to_string();
        } else if self.month_picker.value_as_int() != 0 {
            let jsonified_month = json!({
                "month": self.month_picker.value_as_int()
            });

            return jsonified_month.to_string();
        } else if self.year_picker.value_as_int() != 0 {
            let jsonified_year = json!({
                "year": self.year_picker.value_as_int()
            });

            return jsonified_year.to_string();
        } else {
            return "{}".to_string();
        }
    }

    fn get_submit_button(&self) -> Button {
        return self.submit_btn.clone();
    }

    fn get_summary_text(&self) -> String {
        /* Constructing summary text showing each value selected with its unit, comma separated */
        let mut brief_str = "".to_string();

        let pickers = [
                                            &self.year_picker,
                                            &self.month_picker,
                                            &self.day_picker,
                                            &self.hour_picker,
                                            &self.minute_picker,
                                            &self.second_picker
                                        ];
        let picker_units = ["y", "mo", "d", "h", "m", "s"];

        for (picker, unit) in pickers.iter().zip(picker_units.iter()) {
            // Only show the values which aren't 0
            if picker.value_as_int() != 0 {
                if brief_str != "" {
                    brief_str = format!("{}, {}{}", brief_str, picker.text(), unit);
                } else {
                    brief_str = format!("{}{}", picker.text(), unit);
                }
            }
        }

        return brief_str;
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

fn create_all_disable_callbacks(separate: Vec<(ActionRow, SpinButton)>, hms: Vec<(ActionRow, SpinButton)>) {
    // "separate_rows" is a list of all of the separate options' rows
    let separate_rows: Vec<ActionRow> = separate.iter().map(|t| t.0.clone()).collect();
    
    // "all" is a list of all options' rows and buttons
    let mut all = separate.clone();
    all.extend(hms.iter().cloned());

    /* If one of the separate options (years, months, days) is selected
     * then disable ALL other options */
    for (row, btn) in separate {
        let others: Vec<ActionRow> = all.iter().map(|t| t.0.clone()).filter(|r| *r != row).collect();

        create_disable_callback(btn, others);
    }

    /* If one of the joint options (hours, minute, seconds) is selected,
     * then disable the separate options (years, months, days) */
    for (_, btn) in hms {
        create_disable_callback(btn, separate_rows.clone());
    }
}

fn create_disable_callback(btn: SpinButton, others: Vec<ActionRow>) {
    /* If the given button is not 0, disable all of the rows in "others",
     * but if it is 0, enable all of "others" */
    btn.connect_changed(move | changed | {
        let os = others.clone();
        if changed.value_as_int() == 0 {
            for o in os {
                o.set_sensitive(true);
            }
        } else {
            for o in os {
                o.set_sensitive(false);
            }
        }
    });
}
