use async_channel::Sender;
use gtk4::{Button, Image, glib::{self, object::ObjectExt}, prelude::{BoxExt, ButtonExt, EditableExt, WidgetExt}};
use libadwaita::{ActionRow, ApplicationWindow, EntryRow, HeaderBar, NavigationPage, PreferencesGroup, PreferencesPage, PreferencesRow, SwitchRow, ToolbarView, prelude::{ActionRowExt, AdwDialogExt, EntryRowExt, PreferencesGroupExt, PreferencesPageExt, PreferencesRowExt}};
use sigroute_common::{AutomationTrigger, action_to_icon_name, action_to_name, trigger_to_icon_name, trigger_to_name};

use crate::{app_model::AppModel, automation::{action_menu, delete_menu::DeleteMenu, trigger_menu::{self, TriggerMenu}, trigger_summary}, message::{ModelUpdate::{self, AutomationUpdate}, UIEvent::{self, UpdatedAutomationActivity, UpdatedAutomationName}}};

pub struct AutomationView {
    pub window: ApplicationWindow,
    pub root: NavigationPage,
    pub sender: Sender<UIEvent>,
    pub automation_info: PreferencesPage,
    pub name: EntryRow,
    pub active: SwitchRow,

    pub triggers: PreferencesGroup,
    pub triggers_list: Vec<ActionRow>,
    pub add_trigger_btn: ActionRow,

    pub actions: PreferencesGroup,
    pub actions_list: Vec<ActionRow>,
    pub add_action_btn: ActionRow,
}

impl AutomationView {
    pub fn new(sender: &Sender<UIEvent>, window: &ApplicationWindow) -> Self {
        
        let content_header = HeaderBar::builder()
            .title_widget(&gtk4::Label::builder().use_markup(true).label("<b></b>").halign(gtk4::Align::Start).margin_end(20).margin_start(20).build())
            .build();

        let automation_info = PreferencesPage::new();

        /* Automation details */

        let automation_details_group = PreferencesGroup::builder()
            .title("Details")
            .build();

        let automation_title_entry = EntryRow::builder()
            .title("Name")
            .show_apply_button(true)
            .build();

        let title_row = automation_title_entry.clone();
        let s = sender.clone();
        automation_title_entry.connect_apply(glib::clone!(#[weak] window, move |_| {
            let s = s.clone();
            let title_row = title_row.clone();
            glib::spawn_future_local(async move {
                s.send(UpdatedAutomationName(title_row.text().to_string())).await.unwrap();
            });
            gtk4::prelude::GtkWindowExt::set_focus(&window, None::<&gtk4::Widget>);
        }));

        let automation_title = PreferencesRow::builder()
            .title("Name")
            .child(&automation_title_entry)
            .build();
        automation_details_group.add(&automation_title);

        let automation_status = SwitchRow::builder()
            .title("Active")
            .subtitle("Should the automation run")
            .build();

        let s = sender.clone();
        automation_status.connect_notify_local(Some("active"), move |row, _| {
            let s = s.clone();
            let active = row.is_active();

            glib::spawn_future_local(async move {
                s.send(UpdatedAutomationActivity(active)).await.unwrap();
            });
        });

        automation_details_group.add(&automation_status);

        automation_info.add(&automation_details_group);

        /* Automation Triggers */

        let automation_triggers_group = PreferencesGroup::builder()
            .title("Triggers")
            .build();

        automation_info.add(&automation_triggers_group);

        // "Add Trigger" button

        let add_trigger_row = ActionRow::builder()
            .activatable(true)
            .title("Add Trigger")
            .subtitle("Click to add a new trigger")
            .build();

        let add_trig_img = Image::new();
        add_trig_img.set_icon_name(Some("external-link-symbolic"));

        add_trigger_row.add_suffix(&add_trig_img);

        let window_clone = window.clone();
        let sender_clone = sender.clone();
        add_trigger_row.connect_activated(move |_| {
            trigger_menu::TriggerMenu::new(&sender_clone, &window_clone, false, None);
        });

        automation_triggers_group.add(&add_trigger_row);

        /* Automation Actions */

        let automation_actions_group = PreferencesGroup::builder()
            .title("Actions")
            .build();

        automation_info.add(&automation_actions_group);

        // "Add Action" button
        
        let add_action_row = ActionRow::builder()
            .activatable(true)
            .title("Add Action")
            .subtitle("Click to add a new action")
            .build();

        let add_action_img = Image::new();
        add_action_img.set_icon_name(Some("external-link-symbolic"));

        add_action_row.add_suffix(&add_action_img);

        let menu = action_menu::create_dialog(sender);

        let window_clone = window.clone();
        add_action_row.connect_activated(move |_| {
            menu.present(Some(&window_clone));
        });

        automation_actions_group.add(&add_action_row);

        /* Toolbar and page */

        let content_toolbar = ToolbarView::builder()
            .content(&automation_info)
            .build();
        content_toolbar.add_top_bar(&content_header);

        let content = NavigationPage::builder()
            .child(&content_toolbar)
            .title("Automation")
            .build();

        automation_info.set_visible(false);

        Self {
            window: window.clone(),
            root: content,
            sender: sender.clone(),
            automation_info: automation_info,
            name: automation_title_entry,
            active: automation_status,
            triggers: automation_triggers_group,
            triggers_list: Vec::new(),
            add_trigger_btn: add_trigger_row,
            actions: automation_actions_group,
            actions_list: Vec::new(),
            add_action_btn: add_action_row,
        }
    }

    pub async fn handle_model_update(&mut self, model: &mut AppModel, message: ModelUpdate) {
        self.automation_info.set_visible(true);

        match message {
            AutomationUpdate => {
                // Setting the name for this automation (toggle the apply button to ignore any changes)
                self.name.set_show_apply_button(false);
                self.name.set_text(&model.automations[model.current_index as usize].name);
                self.name.set_show_apply_button(true);

                // Setting whether or not this automation is active
                self.active.set_active(model.automations[model.current_index as usize].active);

                /* === Triggers === */

                // Removing all pre-existing triggers from the last automation
                for trigger_row in &self.triggers_list {
                    self.triggers.remove(trigger_row);
                }
                self.triggers_list.clear();

                // Removing the old "add trigger" button
                self.triggers.remove(&self.add_trigger_btn);

                // Adding all of the new triggers
                for trigger in &model.triggers {
                    // Creating the row, and adding the name and details of the trigger to it
                    let item = ActionRow::new();
                    item.set_title(&trigger_to_name(trigger.trig_type));
                    item.set_subtitle(&trigger_summary::summarise(trigger));

                    // Adding the icon image to the start of the row
                    let icon_image = Image::new();
                    icon_image.set_icon_name(Some(&trigger_to_icon_name(trigger.trig_type)));
                    item.add_prefix(&icon_image);

                    // Adding the edit and delete buttons to the row
                    let button_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
                    button_box.set_valign(gtk4::Align::Center);

                    // Adding the edit button
                    let edit_btn = Button::new();
                    let edit_icon = Image::new();
                    edit_icon.set_icon_name(Some("document-edit-symbolic"));
                    edit_btn.set_child(Some(&edit_icon));

                    edit_btn.set_margin_top(8);
                    edit_btn.set_margin_bottom(8);
                    edit_btn.add_css_class("circular");

                    let trigger_clone = trigger.clone();
                    let sender_clone = self.sender.clone();
                    let window_clone = self.window.clone();
                    edit_btn.connect_clicked(move |_| {
                        update_trigger_handler(trigger_clone.clone(), &sender_clone, &window_clone);
                    });

                    button_box.append(&edit_btn);

                    // Adding the delete button
                    let delete_btn = Button::new();
                    let delete_icon = Image::new();
                    delete_icon.set_icon_name(Some("user-trash-symbolic"));
                    delete_btn.set_child(Some(&delete_icon));

                    delete_btn.set_margin_top(8);
                    delete_btn.set_margin_bottom(8);
                    delete_btn.add_css_class("circular");
                    delete_btn.add_css_class("destructive-action");

                    let sender_clone = self.sender.clone();
                    let window_clone = self.window.clone();
                    let trigger_id = trigger.id;
                    delete_btn.connect_clicked(move |_| {
                        DeleteMenu::new(&sender_clone, &window_clone, trigger_id);
                    });
                    
                    button_box.append(&delete_btn);
            
                    item.add_suffix(&button_box);

                    self.triggers.add(&item);
                    self.triggers_list.push(item);
                }

                // Re-adding the "add trigger" button
                self.triggers.add(&self.add_trigger_btn);

                /* === Actions === */

                // Removing all pre-existing actions from the last automation
                for action_row in &self.actions_list {
                    self.actions.remove(action_row);
                }
                self.actions_list.clear();

                // Removing the old "add trigger" button
                self.actions.remove(&self.add_action_btn);

                // Adding all of the new actions
                for action in &model.actions {
                    let item = ActionRow::new();
                    item.set_title(&action_to_name(action.action_type));

                    let icon_image = Image::new();
                    icon_image.set_icon_name(Some(&action_to_icon_name(action.action_type)));
                    item.add_prefix(&icon_image);
            
                    self.actions.add(&item);
                    self.actions_list.push(item);
                }
                
                // Re-adding the "add action" button
                self.actions.add(&self.add_action_btn);
            }
            _ => {}
        }
    }
}

fn update_trigger_handler(trigger: AutomationTrigger, sender: &Sender<UIEvent>, window: &ApplicationWindow) {
    let _ = TriggerMenu::new(sender, window, true, Some(trigger));
}
