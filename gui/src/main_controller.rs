//! This module acts as the controller for the application, receiving
//! events from the views, accessing the DB, updating the model and
//! dispatching events back to update the views accordingly.

use crate::{api::APIConnection, app_model::AppModel, automation::view::AutomationView, message::{ModelUpdate::{self, AutomationListUpdate}, UIEvent}, sidebar::view::SidebarView};

/// The main controller of the application - owns the model and the
/// views for the app.
pub struct MainController {
    app_model: AppModel,
    sidebar_view: SidebarView,
    automation_view: AutomationView,
}

impl MainController {
    /// Constructor for the controller of the app - requires the connection
    /// to the API, and both of the main views in the application.
    pub async fn new(api_conn: APIConnection, sidebar_view: SidebarView, automation_view: AutomationView) -> Self {
        let model = AppModel::new(api_conn);

        let mut this = Self {
            app_model: model,
            sidebar_view: sidebar_view,
            automation_view: automation_view,
        };

        // Initialising the app to its base state
        this.app_model.update_automations_list().await;
        this.sidebar_view.handle_model_update(&mut this.app_model, AutomationListUpdate).await;

        this
    }

    /// Dispatches a message to both primary views notifying them of an
    /// update to the application model.
    async fn notify_views_of(&mut self, message: ModelUpdate) {
        // This is effectively the same as model notifying each of the views of an event,
        // but the controller is doing it instead (as the controller owns everything)
        self.sidebar_view.handle_model_update(&mut self.app_model, message).await;
        self.automation_view.handle_model_update(&mut self.app_model, message).await;
    }

    /// Handles an incoming message from the views about an action the
    /// user has taken in the GUI.
    pub async fn handle(&mut self, message: UIEvent) {
        match message {
            UIEvent::AddedAutomation => {
                self.app_model.add_new_automation().await;

                self.notify_views_of(ModelUpdate::AutomationListUpdate).await;
                self.notify_views_of(ModelUpdate::AutomationUpdate).await;
            }
            UIEvent::ChangedAutomation(index) => {
                let id = self.app_model.automations[index as usize].id;

                // Update the model's index, ID and lists of triggers and actions, and notify the views of the change
                self.app_model.set_current_automation_id(id);
                self.app_model.set_current_automation_index(index);
                self.app_model.update_triggers_list().await;
                self.app_model.update_actions_list().await;
                self.notify_views_of(ModelUpdate::AutomationUpdate).await;
            }
            UIEvent::UpdatedAutomationName(new_name) => {
                let index = self.app_model.get_current_automation_index() as usize;
                self.app_model.automations[index].name = new_name;

                self.app_model.sync_automation_changes().await;
                self.app_model.update_automations_list().await;

                self.notify_views_of(ModelUpdate::AutomationListUpdate).await;
                self.notify_views_of(ModelUpdate::AutomationUpdate).await;
            }
            UIEvent::UpdatedAutomationActivity(active) => {
                let index = self.app_model.get_current_automation_index() as usize;
                let previous_activity = self.app_model.automations[index].active;

                // Don't bother updating everything if the active state hasn't even changed
                if previous_activity == active {
                    return;
                }

                // Otherwise, update the state in the database and in the GUI
                self.app_model.automations[index].active = active;

                self.app_model.sync_automation_changes().await;
                self.app_model.update_automations_list().await;

                self.notify_views_of(ModelUpdate::AutomationListUpdate).await;
                self.notify_views_of(ModelUpdate::AutomationUpdate).await;
            }
            UIEvent::DeletedAutomation() => {
                self.app_model.delete_automation().await;
                self.app_model.update_automations_list().await;

                self.notify_views_of(ModelUpdate::AutomationListUpdate).await;
                self.notify_views_of(ModelUpdate::AutomationUpdate).await;
            }
            UIEvent::RanAutomation() => {
                self.app_model.run_automation().await;
            }
            UIEvent::AddedTrigger(trig_type, details) => {
                self.app_model.add_trigger(trig_type, details).await;
                self.app_model.update_triggers_list().await;

                self.notify_views_of(ModelUpdate::AutomationUpdate).await;
            }
            UIEvent::UpdatedTrigger(trig_id, new_details) => {
                self.app_model.update_trigger(trig_id, new_details).await;
                self.app_model.update_triggers_list().await;

                self.notify_views_of(ModelUpdate::AutomationUpdate).await;
            }
            UIEvent::DeletedTrigger(trig_id) => {
                self.app_model.delete_trigger(trig_id).await;
                self.app_model.update_triggers_list().await;

                self.notify_views_of(ModelUpdate::AutomationUpdate).await;
            }
            UIEvent::AddedAction(action_type, details) => {
                self.app_model.add_action(action_type, details).await;
                self.app_model.update_actions_list().await;

                self.notify_views_of(ModelUpdate::AutomationUpdate).await;
            }
            UIEvent::UpdatedAction(action_id, new_details) => {
                self.app_model.update_action(action_id, new_details).await;
                self.app_model.update_actions_list().await;

                self.notify_views_of(ModelUpdate::AutomationUpdate).await;
            }
            UIEvent::MoveAction(action_id, direction) => {
                self.app_model.move_action(action_id, direction).await;
                self.app_model.update_actions_list().await;

                self.notify_views_of(ModelUpdate::AutomationUpdate).await;
            }
            UIEvent::DeletedAction(action_id) => {
                self.app_model.delete_action(action_id).await;
                self.app_model.update_actions_list().await;

                self.notify_views_of(ModelUpdate::AutomationUpdate).await;
            }
        }
    }
}
