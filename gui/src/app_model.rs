use sigroute_common::{Automation, AutomationTrigger};

use crate::api::{self, APIConnection};

pub struct AppModel {
    pub api_conn: APIConnection,

    pub automations: Vec<Automation>,
    pub automation_id: i64,
    pub triggers: Vec<AutomationTrigger>,
}

impl AppModel {
    pub async fn update_automations_list(&mut self) {
        let automations_result = api::get_automations(&self.api_conn).await;

        match automations_result {
            Ok(automations) => {
                self.automations = automations;
            }
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            },
        }
    }

    pub async fn update_triggers_list(&mut self) {
        let triggers_result = api::get_automation_triggers(&self.api_conn, self.automation_id).await;

        match triggers_result {
            Ok(triggers) => {
                self.triggers = triggers;
            }
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            },
        }
    }
}
