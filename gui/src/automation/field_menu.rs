use std::{cell::RefCell, rc::Rc};

use async_channel::Sender;
use gtk4::{Button, Image, Label, StringList, glib, prelude::{ButtonExt, WidgetExt}};
use libadwaita::{ActionRow, ApplicationWindow, ComboRow, Dialog, HeaderBar, PreferencesGroup, PreferencesPage, ToolbarView, prelude::{ActionRowExt, AdwDialogExt, ComboRowExt, PreferencesGroupExt, PreferencesPageExt}};
use serde_json::{Map, Value, json};
use sigroute_common::{ACTION_MAX, OptionType, TRIGGER_MAX, action_get_option_details, action_to_name, trigger_get_option_details, trigger_to_name};

use crate::{automation::pickers::{datetime_picker::DateTimePicker, days_picker::DaysPicker, frequency_picker::FrequencyPicker, option_picker::OptionPicker, string_picker::StringPicker, time_picker::TimePicker}, message::UIEvent::{self, AddedAction, AddedTrigger, UpdatedAction, UpdatedTrigger}};

#[derive(PartialEq, Clone, Copy)]
pub enum FieldType {
    Trigger,
    Action
}

#[derive(PartialEq, Clone, Copy)]
pub enum FieldAction {
    Add,
    Edit
}

#[derive(Clone)]
pub struct FieldEditDetails {
    pub id: i64,
    pub field_type: i64,
    pub details: String
}

#[derive(Clone)]
pub struct FieldMenu {
    pub dialog: Dialog,
    pub submit_btn: Button,
    pub selected_type: Rc<RefCell<i64>>,
    pub options: Rc<RefCell<Vec<ActionRow>>>,
    pub mandatory_options_left: Rc<RefCell<u64>>,
    pub json_options: Rc<RefCell<Vec<String>>>,
    pub completed: Rc<RefCell<Vec<bool>>>,
}

impl FieldMenu {
    pub fn new(sender: &Sender<UIEvent>, window: &ApplicationWindow, field_type: FieldType, action: FieldAction, field_to_edit: Option<FieldEditDetails>) -> Self {
        // Ensuring that if edit mode is enabled, a valid trigger has been given to us
        if action == FieldAction::Edit && field_to_edit.is_none() {
            panic!("No field passed to FieldMenu despite edit mode being enabled");
        }

        let menu = Dialog::builder()
            .title(match action {
                FieldAction::Edit => {
                    match field_type {
                        FieldType::Trigger => "Edit Trigger",
                        FieldType::Action => "Edit Action"
                    }
                },
                FieldAction::Add => {
                    match field_type {
                        FieldType::Trigger => "Add Trigger",
                        FieldType::Action => "Add Action"
                    }
                }
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

        /* Creating the first group, allowing for type selection */
        let type_group = PreferencesGroup::builder()
            .title(match field_type {
                FieldType::Trigger => "Trigger",
                FieldType::Action => "Action"
            })
            .build();

        let type_row = ComboRow::builder()
            .title(match field_type {
                FieldType::Trigger => "Trigger Type",
                FieldType::Action => "Action Type"
            })
            .subtitle(match field_type {
                FieldType::Trigger => "What to trigger on",
                FieldType::Action => "What action to perform"
            })
            .build();
        
        // Creating the list of available type names
        let model = StringList::new(&[]);

        match field_type {
            FieldType::Trigger => {
                for i in 1..=TRIGGER_MAX {
                    model.append(&trigger_to_name(i));
                }
            }
            FieldType::Action => {
                for i in 1..=ACTION_MAX {
                    model.append(&action_to_name(i));
                }
            }
        }

        type_row.set_model(Some(&model));
        type_group.add(&type_row);

        // If editing, switching to the correct field and disabling the menu
        if action == FieldAction::Edit {
            // We can use .unwrap as we have already checked that field_to_edit is Some(f)
            let field = field_to_edit.as_ref().unwrap();

            type_row.set_selected((field.field_type - 1) as u32);
            type_row.set_sensitive(false);
        }

        /* Creating the group for the "Add"/"Save Changes" (submit) button */
        let submit_group = PreferencesGroup::new();

        let submit_btn = Button::builder()
            .label(match action {
                FieldAction::Edit => "Save Changes",
                FieldAction::Add => "Add"
            })
            .build();

        submit_btn.add_css_class("success");
        submit_btn.set_sensitive(false);

        submit_group.add(&submit_btn);

        let model = Self {
            dialog: menu,
            submit_btn: submit_btn,
            selected_type: Rc::new(RefCell::new(
                match action {
                    FieldAction::Edit => {
                        // We can use .unwrap as we have already checked that field_to_edit is Some(f)
                        let field = field_to_edit.as_ref().unwrap();

                        field.field_type
                    },
                    FieldAction::Add => 1
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
        let type_row_clone = type_row.clone();
        let field_to_edit_clone = field_to_edit.clone();
        model.submit_btn.connect_clicked(move |_| {
            // Close the dialog
            model_clone.dialog.close();

            // Constructing the final options JSON
            let options = match field_type {
                FieldType::Trigger => trigger_get_option_details(*model_clone.selected_type.borrow()),
                FieldType::Action => action_get_option_details(*model_clone.selected_type.borrow())
            };

            let mut json_map = Map::new();

            for (option, json) in options.iter().zip(model_clone.json_options.borrow().iter()) {
                json_map.insert(option.json_name.clone(), serde_json::from_str(json).unwrap_or_default());
            }

            let options_json = serde_json::Value::Object(json_map);

            // Trigger an event to notify the controller of the new (or update to the) field
            let s = s.clone();
            let type_row_clone = type_row_clone.clone();
            if action == FieldAction::Add {
                glib::spawn_future_local(async move {
                    let event = match field_type {
                        FieldType::Trigger => AddedTrigger((type_row_clone.selected() + 1).into(), options_json.to_string()),
                        FieldType::Action => AddedAction((type_row_clone.selected() + 1).into(), options_json.to_string())
                    };

                    s.send(event).await.unwrap();
                });
            } else {
                // We can use .unwrap as we have already checked that field_to_edit is Some(f)
                let field = field_to_edit_clone.as_ref().unwrap();
                let field_id = field.id;
                glib::spawn_future_local(async move {
                    match field_type {
                        FieldType::Trigger => {
                            s.send(UpdatedTrigger(field_id, options_json.to_string())).await.unwrap();
                        }
                        FieldType::Action => {
                            s.send(UpdatedAction(field_id, options_json.to_string())).await.unwrap();
                        }
                    }
                });
            }
        });

        /* Creating the options group */
        let options_group = PreferencesGroup::builder()
            .title("Options")
            .build();

        if action == FieldAction::Edit {
            // We can use .unwrap as we have already checked that field_to_edit is Some(f)
            let field = field_to_edit.as_ref().unwrap();
            update_options_group(window, options_group.clone(), &model, field_type, action, &field.details);
        } else {
            update_options_group(window, options_group.clone(), &model, field_type, action, "");
        }

        /* Constructing the page in the correct order */
        page.add(&type_group);
        page.add(&options_group);
        page.add(&submit_group);
        toolbar_view.set_content(Some(&page));

        /* Updating the options group when the type changes */
        let model_clone = model.clone();
        let window_clone = window.clone();
        type_row.connect_selected_notify(move |row| {
            // Removing all of the old options
            for option in model_clone.options.borrow().iter() {
                options_group.remove(option);
            }
            model_clone.options.borrow_mut().clear();

            // Updating the selected type
            *model_clone.selected_type.borrow_mut() = 1 + row.selected() as i64;

            // Adding the new options
            update_options_group(&window_clone, options_group.clone(), &model_clone, field_type, action, "");
        });

        /* Showing the dialog */
        model.dialog.present(Some(window));
        
        model
    }

    pub fn update_submit_button(&self) {
        /* If there are no options left which need to be
         * configured, enable the button */
        if *self.mandatory_options_left.borrow() == 0 {
            self.submit_btn.set_sensitive(true);
        } else {
            self.submit_btn.set_sensitive(false);
        }
    }
}

fn update_options_group(window: &ApplicationWindow, group: PreferencesGroup, model: &FieldMenu, field_type: FieldType, action: FieldAction, default_json: &str) {
    let options = match field_type {
        FieldType::Trigger => trigger_get_option_details(*model.selected_type.borrow()),
        FieldType::Action => action_get_option_details(*model.selected_type.borrow())
    };

    // Initialise the vectors storing the resultant json and whether options are completed
    model.json_options.replace(vec!["".to_string(); options.len()]);
    model.completed.replace(vec![false; options.len()]);

    // Reset the number of mandatory options left
    model.mandatory_options_left.replace(0);

    // Get the JSON from the default JSON
    let json_options: Map<String, Value> = serde_json::from_str(default_json).unwrap_or_default();

    let mut index = 0;

    // Add a row for each option for this field
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
        summary.set_wrap(true);
        summary.set_width_chars(20);
        action_row.add_suffix(&summary);

        let icon = Image::new();
        icon.set_icon_name(Some("document-edit-symbolic"));

        action_row.add_suffix(&icon);

        /* If the menu is for editing, pre-populate the options from the default_json */
        if action == FieldAction::Edit {
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
                model_clone.update_submit_button();

                picker.close();
            });
        });

        group.add(&action_row);
        model.options.borrow_mut().push(action_row);

        index += 1;
    }

    // If there are no mandatory choices, then the add button should be enabled
    model.update_submit_button();
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
        OptionType::Time => {
            Box::new(TimePicker::new(window, should_display, json)) as Box<dyn OptionPicker>
        }
        _ => {
            Box::new(StringPicker::new(window, should_display, json)) as Box<dyn OptionPicker>
        }
    }
}
