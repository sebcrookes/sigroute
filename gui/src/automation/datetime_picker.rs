use chrono::{Datelike, Local};
use gtk4::{Button, SpinButton, prelude::{WidgetExt}};
use libadwaita::{ActionRow, ApplicationWindow, Dialog, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{ActionRowExt, AdwDialogExt, PreferencesGroupExt, PreferencesPageExt}};

use crate::automation::option_picker::{OptionPicker};

pub struct DateTimePicker {
    dialog: Dialog,
    day_picker: SpinButton,
    month_picker: SpinButton,
    year_picker: SpinButton,
    hour_picker: SpinButton,
    minute_picker: SpinButton,
    second_picker: SpinButton,
    submit_btn: Button,
}

impl DateTimePicker {
    pub fn new(window: &ApplicationWindow) -> Self {
        let picker = Dialog::builder()
            .title("Date and Time Picker")
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

        /* Creating the two main groups - the date entry group, and the time entry group */
        let date_group = PreferencesGroup::builder()
            .title("Select Date")
            .build();

        let time_group = PreferencesGroup::builder()
            .title("Select Time")
            .build();

        // Getting the current date
        let local_time = Local::now();
        
        /* Adding each of the rows for the date (with today's date as the default value) */

        let (row, day_picker) = create_row("Day", "Select the day", 1, 31);
        day_picker.set_value(local_time.day() as f64);
        date_group.add(&row);

        let (row, month_picker) = create_row("Month", "Select the month", 1, 12);
        month_picker.set_value(local_time.month() as f64);
        date_group.add(&row);

        let (row, year_picker) = create_row("Year", "Select the year", 0, 100000);
        year_picker.set_value(local_time.year() as f64);
        date_group.add(&row);

        /* Adding each of the rows for the time */

        let (row, hour_picker) = create_row("Hour", "Select the hour", 0, 23);
        time_group.add(&row);

        let (row, minute_picker) = create_row("Minute", "Select the minute", 0, 59);
        time_group.add(&row);

        let (row, second_picker) = create_row("Second", "Select the second", 0, 59);
        time_group.add(&row);

        // Creating the "Submit" button
        let submit_group = PreferencesGroup::new();

        let submit_btn = Button::builder()
            .label("Submit")
            .build();

        submit_btn.add_css_class("success");

        submit_group.add(&submit_btn);

        /* Constructing the page and displaying the dialog to the user */

        page.add(&date_group);
        page.add(&time_group);
        page.add(&submit_group);

        toolbar_view.set_content(Some(&page));

        picker.present(Some(window));

        Self {
            dialog: picker,
            day_picker: day_picker,
            month_picker: month_picker,
            year_picker: year_picker,
            hour_picker: hour_picker,
            minute_picker: minute_picker,
            second_picker: second_picker,
            submit_btn: submit_btn,
        }
    }
}

impl OptionPicker for DateTimePicker {
    fn is_now_completed(&self) -> bool {
        return true;
    }

    fn get_json(&self) -> String {
        return "".to_string();
    }

    fn get_submit_button(&self) -> Button {
        return self.submit_btn.clone();
    }

    fn get_summary_text(&self) -> String {
        return format!(
            "{:02}/{:02}/{} {:02}:{:02}:{:02}",
            self.day_picker.value_as_int(),
            self.month_picker.value_as_int(),
            self.year_picker.value_as_int(),
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
