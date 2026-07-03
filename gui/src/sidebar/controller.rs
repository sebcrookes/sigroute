use crate::{app_model::AppModel, message::Message::{self, Initialisation}, sidebar::view::SidebarView};

pub struct SidebarController {
    view: SidebarView,
}

impl SidebarController {
    pub fn new(view: SidebarView) -> Self {
        Self {
            view: view,
        }
    }

    pub async fn render(&mut self, model: &AppModel) {
        let prev_id = self.view.get_current_id();

        self.view.clear_automations();

        let mut i = 0;
        for automation in &model.automations {
            self.view.add_automation(automation.name.clone(), automation.id);

            if automation.id == prev_id {
                self.view.select_by_index(i);
            }

            i += 1;
        }
    }
    
    pub async fn handle(&mut self, model: &mut AppModel, message: &Message) {
        match message {
            Initialisation => {
                model.update_automations_list().await;
                self.render(model).await;
            }
            _ => {}
        }
    }
}
