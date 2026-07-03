use sigroute_common::Automation;

use crate::api::{self, APIConnection};

pub struct AppModel {
    pub api_conn: APIConnection,
    pub automations: Vec<Automation>,
}

impl AppModel {
    pub async fn update_automations_list(&mut self) {
        let automations_result = api::get_automations(&self.api_conn).await;

        match automations_result {
            Ok(automations) => {
                self.automations = automations;
            }
            Err(_) => {},
        }
    }
}
