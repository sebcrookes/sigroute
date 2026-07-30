use sigroute_common::{Automation, AutomationAction, AutomationTrigger};

use crate::api::{self, APIConnection};

pub struct AppModel {
    pub api_conn: APIConnection,

    pub automations: Vec<Automation>,
    pub automation_id: i64,
    pub current_index: i64,
    pub triggers: Vec<AutomationTrigger>,
    pub actions: Vec<AutomationAction>,
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

    pub async fn add_new_automation(&mut self) {
        let new_id_result = api::add_automation(&self.api_conn, "New Automation".to_string()).await;

        let new_id = match new_id_result {
            Ok(new_id) => new_id,
            Err(_) => {
                println!("Error creating new automation");
                std::process::exit(1);
            }
        };

        self.automation_id = new_id;

        self.update_automations_list().await;
        self.update_automation_index().await;

        self.update_triggers_list().await;
        self.update_actions_list().await;
    }

    pub async fn update_automation_index(&mut self) {
        let mut index = 0;
        for automation in &self.automations {
            if automation.id == self.automation_id {
                self.current_index = index;
                return;
            }
            index += 1;
        }

        self.current_index = -1;
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

    pub async fn update_actions_list(&mut self) {
        let actions_result = api::get_automation_actions(&self.api_conn, self.automation_id).await;

        match actions_result {
            Ok(actions) => {
                self.actions = actions;
            }
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            },
        }
    }

    pub async fn sync_automation_changes(&mut self) {
        let _  = api::update_automation(&self.api_conn, self.automations[self.current_index as usize].clone()).await;
    }

    pub async fn add_trigger(&mut self, trig_type: i64, details: String) {
        let _ = api::add_trigger(&self.api_conn, self.automation_id, trig_type, details).await;
    }
}
