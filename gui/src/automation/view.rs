use async_channel::Sender;
use gtk4::{Image, ListBox, SelectionMode, gio::Icon, glib, prelude::{EditableExt, WidgetExt}};
use libadwaita::{ActionRow, ApplicationWindow, EntryRow, HeaderBar, NavigationPage, PreferencesGroup, PreferencesPage, PreferencesRow, ToolbarView, prelude::{ActionRowExt, EntryRowExt, PreferencesGroupExt, PreferencesPageExt, PreferencesRowExt}};
use sigroute_common::{trigger_to_icon_name, trigger_to_name};

use crate::{app_model::AppModel, message::{ModelUpdate::{self, AutomationUpdate}, UIEvent}};

pub struct AutomationView {
    pub root: NavigationPage,
    pub automation_info: PreferencesPage,
    pub name: EntryRow,
    pub triggers: PreferencesGroup,
    pub triggers_list: Vec<ActionRow>,
}

impl AutomationView {
    pub fn new(sender: &Sender<UIEvent>, window: &ApplicationWindow) -> Self {
        
        let content_header = HeaderBar::builder()
            .title_widget(&gtk4::Label::builder().use_markup(true).label("<b></b>").halign(gtk4::Align::Start).margin_end(20).margin_start(20).build())
            .build();

        let automation_info = PreferencesPage::builder()
            .build();

        /* Automation details */

        let automation_details_group = PreferencesGroup::builder()
            .title("Details")
            .build();

        let automation_title_entry = EntryRow::builder()
            .title("Name")
            .show_apply_button(true)
            .build();

        automation_title_entry.connect_apply(glib::clone!(#[weak] window, move |_| {
            gtk4::prelude::GtkWindowExt::set_focus(&window, None::<&gtk4::Widget>);
        }));

        let automation_title = PreferencesRow::builder()
            .title("Name")
            .child(&automation_title_entry)
            .build();
        automation_details_group.add(&automation_title);

        automation_info.add(&automation_details_group);

        /* Automation Triggers */

        let automation_triggers_group = PreferencesGroup::builder()
            .title("Triggers")
            .build();

        automation_info.add(&automation_triggers_group);

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
            triggers: automation_triggers_group,
            triggers_list: Vec::new(),
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

                // Removing all pre-existing triggers from the last automation
                for trigger_row in &self.triggers_list {
                    self.triggers.remove(trigger_row);
                }
                self.triggers_list.clear();

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
            }
            _ => {}
        }
    }
}
