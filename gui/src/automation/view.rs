use async_channel::Sender;
use gtk4::{Image, glib::{self, object::ObjectExt}, prelude::{EditableExt, WidgetExt}};
use libadwaita::{ActionRow, ApplicationWindow, EntryRow, HeaderBar, NavigationPage, PreferencesGroup, PreferencesPage, PreferencesRow, SwitchRow, ToolbarView, prelude::{ActionRowExt, EntryRowExt, PreferencesGroupExt, PreferencesPageExt, PreferencesRowExt}};
use sigroute_common::{action_to_icon_name, action_to_name, trigger_to_icon_name, trigger_to_name};

use crate::{app_model::AppModel, automation::trigger_menu, message::{ModelUpdate::{self, AutomationUpdate}, UIEvent::{self, UpdatedAutomationActivity, UpdatedAutomationName}}};

pub struct AutomationView {
    pub root: NavigationPage,
    pub automation_info: PreferencesPage,
    pub name: EntryRow,
    pub active: SwitchRow,

    pub triggers: PreferencesGroup,
    pub triggers_list: Vec<ActionRow>,
    pub add_trigger_btn: ActionRow,

    pub actions: PreferencesGroup,
    pub actions_list: Vec<ActionRow>,
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

        let add_trigger_row = ActionRow::builder()
            .activatable(true)
            .title("Add New Trigger")
            .subtitle("Click to add a new trigger")
            .build();

        let add_trig_img = Image::new();
        add_trig_img.set_icon_name(Some("external-link-symbolic"));

        add_trigger_row.add_suffix(&add_trig_img);

        let window_clone = window.clone();
        let sender_clone = sender.clone();
        add_trigger_row.connect_activated(move |_| {
            trigger_menu::TriggerMenu::new(&sender_clone, &window_clone);
        });

        automation_triggers_group.add(&add_trigger_row);

        /* Automation Actions */

        let automation_actions_group = PreferencesGroup::builder()
            .title("Actions")
            .build();

        automation_info.add(&automation_actions_group);
        
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
            root: content,
            automation_info: automation_info,
            name: automation_title_entry,
            active: automation_status,
            triggers: automation_triggers_group,
            triggers_list: Vec::new(),
            add_trigger_btn: add_trigger_row,
            actions: automation_actions_group,
            actions_list: Vec::new(),
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
                    let item = ActionRow::new();
                    item.set_title(&trigger_to_name(trigger.trig_type));

                    let icon_image = Image::new();
                    icon_image.set_icon_name(Some(&trigger_to_icon_name(trigger.trig_type)));
                    item.add_prefix(&icon_image);
            
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
            }
            _ => {}
        }
    }
}
