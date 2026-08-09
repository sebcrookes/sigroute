use gtk4::{Button, CheckButton, prelude::{CheckButtonExt, WidgetExt}};
use libadwaita::{ActionRow, ApplicationWindow, Dialog, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{ActionRowExt, AdwDialogExt, PreferencesGroupExt, PreferencesPageExt}};

use crate::automation::option_picker::OptionPicker;

pub struct DaysPicker {
    dialog: Dialog,
    day_btns: Vec<CheckButton>,
    submit_btn: Button,
}

impl DaysPicker {
    pub fn new(window: &ApplicationWindow) -> Self {
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

        let (monday_row, monday_btn) = create_row("Monday");
        days_group.add(&monday_row);

        let (tuesday_row, tuesday_btn) = create_row("Tuesday");
        days_group.add(&tuesday_row);

        let (wednesday_row, wednesday_btn) = create_row("Wednesday");
        days_group.add(&wednesday_row);

        let (thursday_row, thursday_btn) = create_row("Thursday");
        days_group.add(&thursday_row);

        let (friday_row, friday_btn) = create_row("Friday");
        days_group.add(&friday_row);

        let (saturday_row, saturday_btn) = create_row("Saturday");
        days_group.add(&saturday_row);

        let (sunday_row, sunday_btn) = create_row("Sunday");
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

        picker.present(Some(window));

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
        return true;
    }

    fn get_json(&self) -> String {
        return "".to_string();
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

    fn close(&self) {
        self.dialog.close();
    }
}

fn create_row(day: &str) -> (ActionRow, CheckButton) {
    let button = CheckButton::new();
    let row = ActionRow::builder()
        .title(day)
        .build();
    row.add_suffix(&button);

    return (row, button);
}
