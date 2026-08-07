use std::{cell::RefCell, rc::Rc};

use async_channel::Sender;
use gtk4::{Button, Image, Label, StringList, glib, prelude::{ButtonExt, WidgetExt}};
use libadwaita::{ActionRow, ApplicationWindow, ComboRow, Dialog, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{ActionRowExt, AdwDialogExt, ComboRowExt, PreferencesGroupExt, PreferencesPageExt}};
use sigroute_common::{OptionType, TRIGGER_MAX, trigger_get_option_details, trigger_to_name};

use crate::{automation::{datetime_picker::{DateTimePicker}, option_picker::OptionPicker, time_picker::TimePicker}, message::UIEvent::{self, AddedTrigger}};

#[derive(Clone)]
pub struct TriggerMenu {
    pub dialog: Dialog,
    pub add_btn: Button,
    pub mandatory_options_left: Rc<RefCell<u64>>,
    pub json_options: Rc<RefCell<Vec<String>>>,
    pub completed: Rc<RefCell<Vec<bool>>>,
}

impl TriggerMenu {
    pub fn new(sender: &Sender<UIEvent>, window: &ApplicationWindow) -> Self {
        let menu = Dialog::builder()
        .title("Add Trigger")
        .content_width(480)
        .build();

        /* Creating the header, which will provide a close button and the title */
        let header = HeaderBar::builder()
            .build();

        let toolbar_view = ToolbarView::new();
        
        toolbar_view.add_top_bar(&header);
        toolbar_view.set_top_bar_style(libadwaita::ToolbarStyle::Flat);

        menu.set_child(Some(&toolbar_view));

        let page = PreferencesPage::new();

        /* Creating the first group, allowing for trigger type selection */
        let trigger_group = PreferencesGroup::builder()
            .title("Trigger")
            .build();

        let triggers = ComboRow::builder()
            .title("Trigger Type")
            .subtitle("What to trigger on")
            .build();
        
        // Creating the list of available trigger names
        let model = StringList::new(&[]);

        for i in 1..=TRIGGER_MAX {
            model.append(&trigger_to_name(i));
        }

        triggers.set_model(Some(&model));
        trigger_group.add(&triggers);

        /* Creating the group for the "Add" (submit) button */
        let submit_group = PreferencesGroup::new();

        let add_btn = Button::builder()
            .label("Add")
            .build();

        add_btn.add_css_class("success");
        add_btn.set_sensitive(false);

        /* Creating the callback for when the add button is pressed */
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

        let model = Self {
            dialog: menu,
            add_btn: add_btn,
            mandatory_options_left: Rc::new(RefCell::new(0)),
            json_options: Rc::new(RefCell::new(Vec::new())),
            completed: Rc::new(RefCell::new(Vec::new())),
        };

        /* Creating the options group */
        let options_group = create_options_group(window, &model, 1);

        /* Constructing the page in the correct order and showing the dialog */
        page.add(&trigger_group);
        page.add(&options_group);
        page.add(&submit_group);
        toolbar_view.set_content(Some(&page));

        model.dialog.present(Some(window));
        
        model
    }

    pub fn update_add_button(&self) {
        /* If there are no options left which need to be
         * configured, enable the button */
        if *self.mandatory_options_left.borrow() == 0 {
            self.add_btn.set_sensitive(true);
        } else {
            self.add_btn.set_sensitive(false);
        }
    }
}

fn create_options_group(window: &ApplicationWindow, model: &TriggerMenu, selected: i64) -> PreferencesGroup {
    let group = PreferencesGroup::builder()
        .title("Options")
        .build();

    let options = trigger_get_option_details(selected);

    // Initialise the vectors storing the resultant json and whether options are completed
    model.json_options.replace(vec!["".to_string(); options.len()]);
    model.completed.replace(vec![false; options.len()]);

    let mut index = 0;

    // Add a row for each option for this trigger
    for option in options {
        let option_type = option.opt_type;
        let option_mandatory = option.mandatory;

        // Calculating the total number of mandatory options in the list
        if option_mandatory {
            *model.mandatory_options_left.borrow_mut() += 1;
        }

        // Adding "(optional)" to the options which aren't mandatory
        let optional_str = match option_mandatory {
            true => "",
            false => " (optional)",
        };

        let title = format!("{}{}", option.title, optional_str);

        /* Creating the action row and its internals */
        let action_row = ActionRow::builder()
            .activatable(true)
            .title(title)
            .subtitle(option.subtitle)
            .build();
        
        let summary = Label::new(None);
        action_row.add_suffix(&summary);

        let icon = Image::new();
        icon.set_icon_name(Some("document-edit-symbolic"));

        action_row.add_suffix(&icon);

        /* Creating the picker when the action row is clicked */
        let window_clone = window.clone();
        let model_clone = model.clone();
        let summary_clone = summary.clone();

        action_row.connect_activated(move |_| {
            let picker = create_picker(option_type, &window_clone);

            /* Creating the callback for when the submit button is pressed */
            let model_clone = model_clone.clone();
            let summary_clone = summary_clone.clone();
            
            picker.get_submit_button().connect_clicked(move |_| {
                /* If the option wasn't completed and now it is, and this is mandatory, then we should decrement
                 * the number of mandatory options remaining. If the option was completed and not it isn't, and
                 * this is mandatory, then we should increment the number of mandatory options remaining. */

                if !model_clone.completed.borrow()[index] && picker.is_now_completed() && option_mandatory {
                    *model_clone.mandatory_options_left.borrow_mut() -= 1;
                } else if model_clone.completed.borrow()[index] && !picker.is_now_completed() && option_mandatory {
                    *model_clone.mandatory_options_left.borrow_mut() += 1;
                }

                model_clone.completed.borrow_mut()[index] = picker.is_now_completed();

                // Updating the summary text next to this option
                summary_clone.set_text(&picker.get_summary_text());

                // Updating the JSON for this option if it has been completed
                if picker.is_now_completed() {
                    model_clone.json_options.borrow_mut()[index] = picker.get_json();
                } else {
                    model_clone.json_options.borrow_mut()[index] = "".to_string();
                }

                // Updating whether or not the add button is enabled
                model_clone.update_add_button();

                picker.close();
            });
        });

        group.add(&action_row);

        index += 1;
    }

    return group;
}

fn create_picker(picker_type: OptionType, window: &ApplicationWindow) -> Box<dyn OptionPicker> {
    match picker_type {
        OptionType::Time => {
            Box::new(TimePicker::new(window)) as Box<dyn OptionPicker>
        }
        _ => {
            Box::new(DateTimePicker::new(window)) as Box<dyn OptionPicker>
        }
    }
}
