use crate::{app_model::AppModel, automation::view::AutomationView, message::Message};

pub struct AutomationController {
    view: AutomationView,
}

impl AutomationController {
    pub fn new(view: AutomationView) -> Self {
        Self {
            view: view,
        }
    }

    pub async fn render(&mut self, model: &AppModel) {
        
    }

    pub async fn handle(&mut self, model: &mut AppModel, message: &Message) {
        
    }
}
