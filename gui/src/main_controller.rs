use crate::{api::APIConnection, app_model::AppModel, automation::view::AutomationView, message::{ModelUpdate::{self, AutomationListUpdate}, UIEvent}, sidebar::view::SidebarView};

pub struct MainController {
    app_model: AppModel,
    sidebar_view: SidebarView,
    automation_view: AutomationView,
}

impl MainController {
    pub async fn new(api_conn: APIConnection, sidebar_view: SidebarView, automation_view: AutomationView) -> Self {
        let model = AppModel {
            api_conn: api_conn,
            automations: Vec::new(),
            automation_id: -1,
            current_index: -1,
            triggers: Vec::new(),
        };

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

    async fn notify_views_of(&mut self, message: ModelUpdate) {
        // This is effectively the same as model notifying each of the views of an event,
        // but the controller is doing it instead (as the controller owns everything)
        self.sidebar_view.handle_model_update(&mut self.app_model, message).await;
        self.automation_view.handle_model_update(&mut self.app_model, message).await;
    }

    pub async fn handle(&mut self, message: UIEvent) {
        match message {
            UIEvent::ChangedAutomation(index) => {
                let id = self.app_model.automations[index as usize].id;

                // Update the model's index, ID and lists of triggers and actions, and notify the views of the change
                self.app_model.automation_id = id;
                self.app_model.current_index = index;
                self.app_model.update_triggers_list().await;
                self.notify_views_of(ModelUpdate::AutomationUpdate).await;
            }
            _ => {}
        }
    }
}
