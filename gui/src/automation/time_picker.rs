use gtk4::{Button, SpinButton, prelude::{EditableExt, WidgetExt}};
use libadwaita::{ActionRow, ApplicationWindow, Dialog, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{ActionRowExt, AdwDialogExt, PreferencesGroupExt, PreferencesPageExt}};

use crate::automation::option_picker::OptionPicker;

pub struct TimePicker {
    dialog: Dialog,
    year_picker: SpinButton,
    month_picker: SpinButton,
    day_picker: SpinButton,
    hour_picker: SpinButton,
    minute_picker: SpinButton,
    second_picker: SpinButton,
    submit_btn: Button,
}

impl TimePicker {
    pub fn new(window: &ApplicationWindow) -> Self {
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

        /* Creating the time selection group */

        let time_group = PreferencesGroup::builder()
            .title("Select Time")
            .build();

        // Adding a hint as to how the selector works
        time_group.set_description(Some("The resultant time is the following values added together"));
        
        /* Adding each of the rows for the individual units of time */

        let (row, year_picker) = create_row("Years", "Number of years", 0, 1000);
        time_group.add(&row);

        let (row, month_picker) = create_row("Months", "Number of months", 0, 1000);
        time_group.add(&row);

        let (row, day_picker) = create_row("Days", "Number of days", 0, 1000);
        time_group.add(&row);

        let (row, hour_picker) = create_row("Hours", "Number of hours", 0, 1000);
        time_group.add(&row);

        let (row, minute_picker) = create_row("Minutes", "Number of minutes", 0, 59);
        time_group.add(&row);

        let (row, second_picker) = create_row("Seconds", "Number of seconds", 0, 59);
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

        picker.present(Some(window));

        Self {
            dialog: picker,
            year_picker: year_picker,
            month_picker: month_picker,
            day_picker: day_picker,
            hour_picker: hour_picker,
            minute_picker: minute_picker,
            second_picker: second_picker,
            submit_btn: submit_btn,
        }
    }
}

impl OptionPicker for TimePicker {
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

    fn get_json(&self) -> String {
        return "".to_string();
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
