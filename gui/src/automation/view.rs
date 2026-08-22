//! This module provides the view for the currently selected
//! automation. It displays details about the automation,
//! such as the name, active status etc., the triggers for
//! that automation, and the actions taken when that automation
//! is run. Updates are handled via handle_model_update, and
//! messages can be sent to the controller via the UIEvent
//! sender.

use async_channel::Sender;
use gtk4::{Box, Button, Image, Label, Orientation, glib::{self, object::ObjectExt}, prelude::{BoxExt, ButtonExt, EditableExt, WidgetExt}};
use libadwaita::{ActionRow, ApplicationWindow, EntryRow, HeaderBar, NavigationPage, PreferencesGroup, PreferencesPage, PreferencesRow, SwitchRow, ToolbarView, prelude::{ActionRowExt, EntryRowExt, PreferencesGroupExt, PreferencesPageExt, PreferencesRowExt}};
use sigroute_common::{AutomationAction, AutomationTrigger, MoveDirection, action_to_icon_name, trigger_to_icon_name, trigger_to_name};

use crate::{app_model::AppModel, automation::{action_summary, delete_menu::{DeleteMenu, DeleteType}, field_menu::{FieldAction, FieldEditDetails, FieldMenu, FieldType}, trigger_summary}, message::{ModelUpdate::{self, AutomationUpdate}, UIEvent::{self, MoveAction, RanAutomation, UpdatedAutomationActivity, UpdatedAutomationName}}};

/// UI component to display information about the currently
/// selected automation.
pub struct AutomationView {
    pub window: ApplicationWindow,
    pub header: HeaderBar,
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
    /// Creates a new automation view, which displays information
    /// about automations - requires the UI event sender, and the
    /// window to display the component in.
    pub fn new(sender: &Sender<UIEvent>, window: &ApplicationWindow) -> Self {
        
        let content_header = HeaderBar::builder()
            .title_widget(&gtk4::Label::builder().use_markup(true).label("<b></b>").halign(gtk4::Align::Start).margin_end(20).margin_start(20).build())
            .build();

        let automation_info = PreferencesPage::new();

        /* === Automation details section === */

        let automation_details_group = PreferencesGroup::builder()
            .title("Details")
            .build();

        // Automation name field

        let automation_title_entry = EntryRow::builder()
            .title("Name")
            .show_apply_button(true)
            .build();

        // Registering a callback to update the automation name when the
        // apply button is clicked
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

        // Automation status slider

        let automation_status = SwitchRow::builder()
            .title("Active")
            .subtitle("Should the automation run")
            .build();

        // Registering a callback to update the daemon when the active status changes
        let s = sender.clone();
        automation_status.connect_notify_local(Some("active"), move |row, _| {
            let s = s.clone();
            let active = row.is_active();

            glib::spawn_future_local(async move {
                s.send(UpdatedAutomationActivity(active)).await.unwrap();
            });
        });

        automation_details_group.add(&automation_status);

        // Adding the automation options
        
        let automation_options_row = ActionRow::builder()
            .title("Options")
            .subtitle("Options for managing the automation")
            .build();

        // Creating the box containing the option buttons
        let automation_option_btns = Box::new(Orientation::Horizontal, 0);
        automation_option_btns.add_css_class("linked");
        automation_option_btns.set_valign(gtk4::Align::Center);
        automation_option_btns.set_margin_top(8);
        automation_option_btns.set_margin_bottom(8);

        // Automation run button
        let automation_run = Button::new();
        automation_run.set_child(Some(&Label::new(Some("Run Manually"))));
        automation_run.set_hexpand(false);

        // Sending a notification to the controller that the run button has been pressed
        let sender_clone = sender.clone();
        automation_run.connect_clicked(move |_| {
            let sender_clone: Sender<UIEvent> = sender_clone.clone();
            glib::spawn_future_local(async move {
                sender_clone.send(RanAutomation()).await.unwrap();
            });
        });

        automation_option_btns.append(&automation_run);

        // Automation delete button
        let automation_delete = Button::from_icon_name("user-trash-symbolic");
        automation_delete.add_css_class("destructive-action");
        automation_delete.set_hexpand(false);

        // Constructing a deletion confirmation menu when delete is pressed
        let sender_clone = sender.clone();
        let window_clone = window.clone();
        automation_delete.connect_clicked(move |_| {
            let sender_clone: Sender<UIEvent> = sender_clone.clone();
            let window_clone = window_clone.clone();
            glib::spawn_future_local(async move {
                let deletion_menu = DeleteMenu::new(&sender_clone, DeleteType::Automation, None);
                deletion_menu.display(&window_clone);
            });
        });

        automation_option_btns.append(&automation_delete);

        automation_options_row.add_suffix(&automation_option_btns);
        automation_details_group.add(&automation_options_row);
        automation_info.add(&automation_details_group);

        /* === Automation triggers section === */

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

        // Displaying a new trigger menu when the add button is pressed
        let window_clone = window.clone();
        let sender_clone = sender.clone();
        add_trigger_row.connect_activated(move |_| {
            FieldMenu::new(&sender_clone, &window_clone, FieldType::Trigger, FieldAction::Add, None);
        });

        automation_triggers_group.add(&add_trigger_row);

        /* === Automation actions section === */

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

        // Displaying a new action menu when the add button is pressed
        let sender_clone = sender.clone();
        let window_clone = window.clone();
        add_action_row.connect_activated(move |_| {
            FieldMenu::new(&sender_clone, &window_clone, FieldType::Action, FieldAction::Add, None);
        });

        automation_actions_group.add(&add_action_row);

        /* === Toolbar and navigation page === */

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
            header: content_header,
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

    /// Gets the root element (NavigationPage) of this view.
    pub fn get_root(&self) -> NavigationPage {
        return self.root.clone();
    }

    /// Performs actions based on a notification sent to the
    /// view by the controller.
    pub async fn handle_model_update(&mut self, model: &mut AppModel, message: ModelUpdate) {
        match message {
            AutomationUpdate => {
                self.render(model);
            }
            _ => {}
        }
    }

    /// Re-draws the widget based on the provided updated
    /// model data.
    fn render(&mut self, model: &mut AppModel) {
        // If the current index is -1, no automation is selected
        self.automation_info.set_visible(model.current_index != -1);
        if model.current_index == -1 {
            self.header.set_title_widget(Some(&Label::new(None)));
            return;
        }

        // Setting the title bar for this automation
        let automation_name = &model.automations[model.current_index as usize].name;
        self.header.set_title_widget(Some(&gtk4::Label::builder().use_markup(true).label(format!("<b>{automation_name}</b>")).halign(gtk4::Align::Start).margin_end(20).margin_start(20).build()));

        // Setting the name for this automation (toggle the apply button to ignore any changes)
        self.name.set_show_apply_button(false);
        self.name.set_text(automation_name);
        self.name.set_show_apply_button(true);

        // Setting whether or not this automation is active
        let automation_active = model.automations[model.current_index as usize].active;
        self.active.set_active(automation_active);

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
            let item = self.create_trigger_row(trigger);

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

        // Removing the old "add action" button
        self.actions.remove(&self.add_action_btn);

        let mut action_index = 0;

        // Adding all of the new actions
        for action in &model.actions {
            let row = self.create_action_row(
                action,
                action_index == 0,
                action_index == model.actions.len() - 1
            );

            self.actions.add(&row);
            self.actions_list.push(row);

            action_index += 1;
        }
        
        // Re-adding the "add action" button
        self.actions.add(&self.add_action_btn);
    }

    /// Creates an ActionRow for the provided trigger, displaying
    /// its name, summary, icon, and buttons for editing and
    /// deleting the trigger.
    fn create_trigger_row(&self, trigger: &AutomationTrigger) -> ActionRow {
        // Creating the row, and adding the name and details of the trigger to it
        let row = ActionRow::new();
        row.set_title(&trigger_to_name(trigger.trig_type));
        row.set_subtitle(&trigger_summary::summarise(trigger));

        // Adding the icon image to the start of the row
        let icon_image = Image::new();
        icon_image.set_icon_name(Some(&trigger_to_icon_name(trigger.trig_type)));
        row.add_prefix(&icon_image);

        // Adding the edit and delete buttons to the row
        let button_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        button_box.set_valign(gtk4::Align::Center);
        button_box.add_css_class("linked");

        // Adding the edit button
        let edit_btn = Button::new();
        let edit_icon = Image::new();
        edit_icon.set_icon_name(Some("document-edit-symbolic"));
        edit_btn.set_child(Some(&edit_icon));

        edit_btn.set_margin_top(8);
        edit_btn.set_margin_bottom(8);

        // Registering a callback to display the edit screen when pressed
        let trigger_clone = trigger.clone();
        let sender_clone = self.sender.clone();
        let window_clone = self.window.clone();
        edit_btn.connect_clicked(move |_| {
            edit_trigger_handler(trigger_clone.clone(), &sender_clone, &window_clone);
        });

        button_box.append(&edit_btn);

        // Adding the delete button
        let delete_btn = Button::new();
        let delete_icon = Image::new();
        delete_icon.set_icon_name(Some("user-trash-symbolic"));
        delete_btn.set_child(Some(&delete_icon));

        delete_btn.set_margin_top(8);
        delete_btn.set_margin_bottom(8);
        delete_btn.add_css_class("destructive-action");

       // Registering a callback to display a delete confirmation when pressed 
        let sender_clone = self.sender.clone();
        let window_clone = self.window.clone();
        let trigger_id = trigger.id;
        delete_btn.connect_clicked(move |_| {
            let deletion_menu = DeleteMenu::new(&sender_clone, DeleteType::Trigger, Some(trigger_id));
            deletion_menu.display(&window_clone);
        });
        
        button_box.append(&delete_btn);

        row.add_suffix(&button_box);

        return row;
    }

    /// Creates an ActionRow for the provided action, displaying
    /// a summary of its action, an icon, move up/down buttons,
    /// an edit button and a delete button.
    fn create_action_row(&self, action: &AutomationAction, is_first: bool, is_last: bool) -> ActionRow {
        // Creating the row, and setting its title and icon image
        let row = ActionRow::new();
        row.set_title(&action_summary::summarise(action));

        let icon_image = Image::new();
        icon_image.set_icon_name(Some(&action_to_icon_name(action.action_type)));
        row.add_prefix(&icon_image);

        /* Adding the up and down buttons to the row */
        let up_down_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        up_down_box.set_valign(gtk4::Align::Center);
        up_down_box.add_css_class("linked");

        // Constructing the up button
        let up_btn = Button::new();
        let up_image = Image::new();
        up_image.set_icon_name(Some("go-up-symbolic"));
        up_btn.set_child(Some(&up_image));
        up_btn.set_margin_top(8);
        up_btn.set_margin_bottom(8);
        up_down_box.append(&up_btn);

        // Disable the up button if this is the first action
        if is_first {
            up_btn.set_sensitive(false);
        }

        // Adding the callback for when the up button is clicked
        let sender_clone = self.sender.clone();
        let action_id = action.id;
        up_btn.connect_clicked(move |_| {
            let s = sender_clone.clone();

            glib::spawn_future_local(async move {
                s.send(MoveAction(action_id, MoveDirection::Up)).await.unwrap();
            });
        });

        // Constructing the down button
        let down_btn = Button::new();
        let down_image = Image::new();
        down_image.set_icon_name(Some("go-down-symbolic"));
        down_btn.set_child(Some(&down_image));
        down_btn.set_margin_top(8);
        down_btn.set_margin_bottom(8);
        up_down_box.append(&down_btn);

        // Disable the down button if this is the last action
        if is_last {
            down_btn.set_sensitive(false);
        }

        // Adding the callback for when the down button is clicked
        let sender_clone = self.sender.clone();
        let action_id = action.id;
        down_btn.connect_clicked(move |_| {
            let s = sender_clone.clone();

            glib::spawn_future_local(async move {
                s.send(MoveAction(action_id, MoveDirection::Down)).await.unwrap();
            });
        });

        row.add_suffix(&up_down_box);

        /* Adding the delete button to the row */
        let misc_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        misc_box.set_valign(gtk4::Align::Center);
        misc_box.set_margin_start(2);
        misc_box.add_css_class("linked");

        // Adding the edit button
        let edit_btn = Button::new();
        let edit_icon = Image::new();
        edit_icon.set_icon_name(Some("document-edit-symbolic"));
        edit_btn.set_child(Some(&edit_icon));

        edit_btn.set_margin_top(8);
        edit_btn.set_margin_bottom(8);

        // Registering a callback to display the edit menu when pressed
        let action_clone = action.clone();
        let sender_clone = self.sender.clone();
        let window_clone = self.window.clone();
        edit_btn.connect_clicked(move |_| {
            edit_action_handler(action_clone.clone(), &sender_clone, &window_clone);
        });

        misc_box.append(&edit_btn);

        // Adding the delete button
        let delete_btn = Button::new();
        let delete_icon = Image::new();
        delete_icon.set_icon_name(Some("user-trash-symbolic"));
        delete_btn.set_child(Some(&delete_icon));

        delete_btn.set_margin_top(8);
        delete_btn.set_margin_bottom(8);
        delete_btn.add_css_class("destructive-action");

        // Registering a callback to display a deletion confirmation menu when pressed
        let sender_clone = self.sender.clone();
        let window_clone = self.window.clone();
        let action_id = action.id;
        delete_btn.connect_clicked(move |_| {
            let deletion_menu = DeleteMenu::new(&sender_clone, DeleteType::Action, Some(action_id));
            deletion_menu.display(&window_clone);
        });

        misc_box.append(&delete_btn);

        row.add_suffix(&misc_box);

        return row;
    }
}

/// Callback for when a trigger edit button is pressed
/// (provided the trigger, UI event sender and window
/// as arguments). Opens the FieldMenu in edit mode.
fn edit_trigger_handler(trigger: AutomationTrigger, sender: &Sender<UIEvent>, window: &ApplicationWindow) {
    let field_edit_details = FieldEditDetails {
        id: trigger.id,
        field_type: trigger.trig_type,
        details: trigger.details
    };

    let _ = FieldMenu::new(sender, window, FieldType::Trigger, FieldAction::Edit, Some(field_edit_details));
}

/// Callback for when an action edit button is pressed
/// (provided the action, UI event sender and window as
/// arguments). Opens the FieldMenu in edit mode.
fn edit_action_handler(action: AutomationAction, sender: &Sender<UIEvent>, window: &ApplicationWindow) {
    let field_edit_details = FieldEditDetails {
        id: action.id,
        field_type: action.action_type,
        details: action.details
    };

    let _ = FieldMenu::new(sender, window, FieldType::Action, FieldAction::Edit, Some(field_edit_details));
}
