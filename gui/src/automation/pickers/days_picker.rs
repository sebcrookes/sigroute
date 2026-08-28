//! This module allows for days of the week to be
//! selected as an option for FieldMenus.

use std::collections::HashMap;

use gtk4::{Button, CheckButton, prelude::{CheckButtonExt, WidgetExt}};
use libadwaita::{Dialog, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{AdwDialogExt, PreferencesGroupExt, PreferencesPageExt}};
use serde_json::{Map, json};

use crate::automation::pickers::misc::create_chkbtn_row;

use super::option_picker::OptionPicker;

/// UI component allowing for the selection of one
/// or more days of the week using libadwaita
/// CheckButtons.
pub struct DaysPicker {
    dialog: Dialog,
    day_btns: Vec<CheckButton>,
    submit_btn: Button,
}

impl DaysPicker {
    /// Constructs a new DaysPicker allowing for the user to
    /// select from multiple (one or more) days of the week -
    /// takes in JSON to allow for the pre-population of the
    /// fields. See get_json for the required format of the
    /// JSON.
    pub fn new(json: String) -> Self {
        let picker = Dialog::builder()
            .title("Days Picker")
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

        /* Creating the day selection group */

        let days_group = PreferencesGroup::builder()
            .title("Select Days")
            .build();

        // Adding a hint as to how the selector works
        days_group.set_description(Some("You can pick multiple days of the week from the list"));
        
        /* Adding each of the rows for the individual days */

        // Getting whether or not each should be enabled from the provided JSON
        let jsonified: HashMap<String, i64> = serde_json::from_str(&json).unwrap_or_default();
        let mut enabled = [false; 7];

        for i in 1..=7 {
            if let Some(val) = jsonified.get(&format!("{}", i)) {
                enabled[i - 1] = *val == 1;
            }
        }

        // Constructing the rows
        
        let (monday_row, monday_btn) = create_chkbtn_row("Monday", None);
        monday_btn.set_active(enabled[0]);
        days_group.add(&monday_row);

        let (tuesday_row, tuesday_btn) = create_chkbtn_row("Tuesday", None);
        tuesday_btn.set_active(enabled[1]);
        days_group.add(&tuesday_row);

        let (wednesday_row, wednesday_btn) = create_chkbtn_row("Wednesday", None);
        wednesday_btn.set_active(enabled[2]);
        days_group.add(&wednesday_row);

        let (thursday_row, thursday_btn) = create_chkbtn_row("Thursday", None);
        thursday_btn.set_active(enabled[3]);
        days_group.add(&thursday_row);

        let (friday_row, friday_btn) = create_chkbtn_row("Friday", None);
        friday_btn.set_active(enabled[4]);
        days_group.add(&friday_row);

        let (saturday_row, saturday_btn) = create_chkbtn_row("Saturday", None);
        saturday_btn.set_active(enabled[5]);
        days_group.add(&saturday_row);

        let (sunday_row, sunday_btn) = create_chkbtn_row("Sunday", None);
        sunday_btn.set_active(enabled[6]);
        days_group.add(&sunday_row);
        
        // Creating the "Submit" button
        let submit_group = PreferencesGroup::new();

        let submit_btn = Button::builder()
            .label("Submit")
            .build();

        submit_btn.add_css_class("success");

        submit_group.add(&submit_btn);

        /* Constructing the page and displaying the dialog to the user */

        page.add(&days_group);
        page.add(&submit_group);

        toolbar_view.set_content(Some(&page));

        Self {
            dialog: picker,
            day_btns: Vec::from([
                                    monday_btn,
                                    tuesday_btn,
                                    wednesday_btn,
                                    thursday_btn,
                                    friday_btn,
                                    saturday_btn,
                                    sunday_btn
                                ]),
            submit_btn: submit_btn,
        }
    }
}

impl OptionPicker for DaysPicker {
    fn is_now_completed(&self) -> bool {
        // A checkbox must be selected to complete this option
        for button in &self.day_btns {
            if button.is_active() {
                return true;
            }
        }

        return false;
    }

    /// Gets the current state of the picker in JSON, and returns this as
    /// a string.
    /// 
    /// The format for DaysPicker is:
    /// {
    ///     "1": 0,
    ///     "2": 1,
    ///     "7": 1
    /// }
    /// 
    /// The first number indicates the day of the week ("1" = Monday, "7" = Sunday).
    /// If that field has a 0, that means it is "repeat never". If a field has a 1,
    /// that means "repeat every week". Any other value is currently invalid.
    fn get_json(&self) -> String {
        let mut data = Map::new();

        let mut index = 0;
        for button in &self.day_btns {
            let active = if button.is_active() {1} else {0};
            data.insert(format!("{}", index + 1), json!(active));
  
            index += 1;
        }

        return serde_json::Value::Object(data).to_string();
    }

    fn get_submit_button(&self) -> Button {
        return self.submit_btn.clone();
    }

    fn get_summary_text(&self) -> String {
        let phrases = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

        let mut result = "".to_string();
        let mut index = 0;
        for button in &self.day_btns {
            if button.is_active() {
                if result == "" {
                    result = phrases[index].to_string();
                } else {
                    result = format!("{}, {}", result, phrases[index]);
                }
            }

            index += 1;
        }

        return result;
    }

    fn display(&self, parent: &Dialog) {
        self.dialog.present(Some(parent));
    }

    fn close(&self) {
        self.dialog.close();
    }
}
