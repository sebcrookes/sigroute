use crate::{api::{APIConnection}, app_model::AppModel, automation::controller::AutomationController, message::Message::{self, Initialisation}, sidebar::controller::SidebarController};

pub struct MainController {
    app_model: AppModel,
    sidebar_controller: SidebarController,
    automation_controller: AutomationController,
}

impl MainController {
    pub async fn new(api_conn: APIConnection, sidebar_controller: SidebarController, automation_controller: AutomationController) -> Self {

        let model = AppModel {
            api_conn: api_conn,
            automations: Vec::new(),
        };

        let mut this = Self {
            app_model: model,
            sidebar_controller: sidebar_controller,
            automation_controller: automation_controller,
        };

        // Telling the controllers to initialise themselves
        this.sidebar_controller.handle(&mut this.app_model, &Initialisation).await;
        this.automation_controller.handle(&mut this.app_model, &Initialisation).await;

        this
    }

    pub async fn handle(&mut self, message: Message) {
        self.sidebar_controller.handle(&mut self.app_model, &message).await;
        self.automation_controller.handle(&mut self.app_model, &message).await;
    }
}
