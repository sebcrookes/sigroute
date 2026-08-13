use std::{cell::RefCell, rc::Rc};

use async_channel::Sender;
use gtk4::{Button, Image, Label, StringList, glib, prelude::{ButtonExt, WidgetExt}};
use libadwaita::{ActionRow, ApplicationWindow, ComboRow, Dialog, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{ActionRowExt, AdwDialogExt, ComboRowExt, PreferencesGroupExt, PreferencesPageExt}};
use serde_json::{Map, Value, json};
use sigroute_common::{AutomationTrigger, OptionType, TRIGGER_MAX, trigger_get_option_details, trigger_to_name};

use crate::{automation::{datetime_picker::DateTimePicker, days_picker::DaysPicker, frequency_picker::FrequencyPicker, option_picker::OptionPicker, time_picker::TimePicker}, message::UIEvent::{self, AddedTrigger, UpdatedTrigger}};

#[derive(Clone)]
pub struct TriggerMenu {
    pub dialog: Dialog,
    pub add_btn: Button,
    pub selected_trigger: Rc<RefCell<i64>>,
    pub options: Rc<RefCell<Vec<ActionRow>>>,
    pub mandatory_options_left: Rc<RefCell<u64>>,
    pub json_options: Rc<RefCell<Vec<String>>>,
    pub completed: Rc<RefCell<Vec<bool>>>,
}

impl TriggerMenu {
    pub fn new(sender: &Sender<UIEvent>, window: &ApplicationWindow, is_editing: bool, trigger_to_edit: Option<AutomationTrigger>) -> Self {
        // Ensuring that if edit mode is enabled, a valid trigger has been given to us
        if is_editing && trigger_to_edit.is_none() {
            panic!("No trigger passed to TriggerMenu despite edit mode being enabled");
        }

        let menu = Dialog::builder()
            .title(match is_editing {
                true => "Edit Trigger",
                false => "Add Trigger"
            })
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

        // If editing, switching to the correct trigger and disabling the menu
        if is_editing {
            // We can use .unwrap as we have already checked that trigger_to_edit is Some(t)
            let trigger = trigger_to_edit.as_ref().unwrap();

            triggers.set_selected((trigger.trig_type - 1) as u32);
            triggers.set_sensitive(false);
        }

        /* Creating the group for the "Add"/"Submit Changes" (submit) button */
        let submit_group = PreferencesGroup::new();

        let add_btn = Button::builder()
            .label(match is_editing {
                true => "Submit Changes",
                false => "Add"
            })
            .build();

        add_btn.add_css_class("success");
        add_btn.set_sensitive(false);

        submit_group.add(&add_btn);

        let model = Self {
            dialog: menu,
            add_btn: add_btn,
            selected_trigger: Rc::new(RefCell::new(
                match is_editing {
                    true => {
                        // We can use .unwrap as we have already checked that trigger_to_edit is Some(t)
                        let trigger = trigger_to_edit.as_ref().unwrap();

                        trigger.trig_type
                    },
                    false => 1
                }
            )),
            options: Rc::new(RefCell::new(Vec::new())),
            mandatory_options_left: Rc::new(RefCell::new(0)),
            json_options: Rc::new(RefCell::new(Vec::new())),
            completed: Rc::new(RefCell::new(Vec::new())),
        };

        /* Creating the callback for when the add button is pressed */
        let model_clone = model.clone();
        let s = sender.clone();
        let triggers_clone = triggers.clone();
        let trigger_to_edit_clone = trigger_to_edit.clone();
        model.add_btn.connect_clicked(move |_| {
            // Close the dialog
            model_clone.dialog.close();

            // Constructing the final options JSON
            let options = trigger_get_option_details(*model_clone.selected_trigger.borrow());

            let mut json_map = Map::new();

            for (option, json) in options.iter().zip(model_clone.json_options.borrow().iter()) {
                json_map.insert(option.json_name.clone(), serde_json::from_str(json).unwrap_or_default());
            }

            let options_json = serde_json::Value::Object(json_map);

            // Trigger an event to notify the controller of the new (or update to the) trigger
            let s = s.clone();
            let triggers_clone = triggers_clone.clone();
            if !is_editing {
                glib::spawn_future_local(async move {
                    s.send(AddedTrigger((triggers_clone.selected() + 1).into(), options_json.to_string())).await.unwrap();
                });
            } else {
                // We can use .unwrap as we have already checked that trigger_to_edit is Some(t)
                let trigger = trigger_to_edit_clone.as_ref().unwrap();
                let trigger_id = trigger.id;
                glib::spawn_future_local(async move {
                    s.send(UpdatedTrigger(trigger_id, options_json.to_string())).await.unwrap();
                });
            }
        });

        /* Creating the options group */
        let options_group = PreferencesGroup::builder()
            .title("Options")
            .build();

        if is_editing {
            // We can use .unwrap as we have already checked that trigger_to_edit is Some(t)
            let trigger = trigger_to_edit.as_ref().unwrap();
            update_options_group(window, options_group.clone(), &model, trigger.trig_type, is_editing, &trigger.details);
        } else {
            update_options_group(window, options_group.clone(), &model, 1, is_editing, "");
        }

        /* Constructing the page in the correct order */
        page.add(&trigger_group);
        page.add(&options_group);
        page.add(&submit_group);
        toolbar_view.set_content(Some(&page));

        /* Updating the options group when the trigger type changes */
        let model_clone = model.clone();
        let window_clone = window.clone();
        triggers.connect_selected_notify(move |row| {
            // Removing all of the old options
            for option in model_clone.options.borrow().iter() {
                options_group.remove(option);
            }
            model_clone.options.borrow_mut().clear();

            // Updating the selected trigger
            *model_clone.selected_trigger.borrow_mut() = 1 + row.selected() as i64;

            // Adding the new options
            update_options_group(&window_clone, options_group.clone(), &model_clone, 1 + row.selected() as i64, false, "");
        });

        /* Showing the dialog */
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

fn update_options_group(window: &ApplicationWindow, group: PreferencesGroup, model: &TriggerMenu, selected: i64, is_editing: bool, default_json: &str) {
    let options = trigger_get_option_details(selected);

    // Initialise the vectors storing the resultant json and whether options are completed
    model.json_options.replace(vec!["".to_string(); options.len()]);
    model.completed.replace(vec![false; options.len()]);

    // Reset the number of mandatory options left
    model.mandatory_options_left.replace(0);

    // Get the JSON from the default JSON
    let json_options: Map<String, Value> = serde_json::from_str(default_json).unwrap_or_default();

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

        /* If the menu is for editing, pre-populate the options from the default_json */
        if is_editing {
            if json_options.contains_key(&option.json_name) {
                // Creating a temporary picker so that we can extract the summary text and JSON
                let picker = create_picker(option_type, &window, false, json!(json_options.get(&option.json_name)).to_string());
            
                summary.set_text(&picker.get_summary_text());

                if picker.is_now_completed() {
                    model.json_options.borrow_mut()[index] = picker.get_json();
                    model.completed.borrow_mut()[index] = true;

                    if option.mandatory {
                        *model.mandatory_options_left.borrow_mut() -= 1;
                    }
                }

                // We don't need to close the picker since it was never presented
            }
        }

        /* Creating the picker when the action row is clicked */
        let window_clone = window.clone();
        let model_clone = model.clone();
        let summary_clone = summary.clone();

        action_row.connect_activated(move |_| {
            let picker = create_picker(option_type, &window_clone, true, model_clone.json_options.borrow()[index].clone());

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
        model.options.borrow_mut().push(action_row);

        index += 1;
    }

    // If there are no mandatory choices, then the add button should be enabled
    model.update_add_button();
}

fn create_picker(picker_type: OptionType, window: &ApplicationWindow, should_display: bool, json: String) -> Box<dyn OptionPicker> {
    match picker_type {
        OptionType::Frequency => {
            Box::new(FrequencyPicker::new(window, should_display, json)) as Box<dyn OptionPicker>
        }
        OptionType::DateTime => {
            Box::new(DateTimePicker::new(window, should_display, json)) as Box<dyn OptionPicker>
        }
        OptionType::Days => {
            Box::new(DaysPicker::new(window, should_display, json)) as Box<dyn OptionPicker>
        }
        _ => {
            Box::new(TimePicker::new(window, should_display, json)) as Box<dyn OptionPicker>
        }
    }
}
