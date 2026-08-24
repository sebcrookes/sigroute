//! This module contains the code for the app model, which stores
//! the current state of the GUI, and makes calls to the daemon
//! to update that data.

use sigroute_common::{Automation, AutomationAction, AutomationTrigger, MoveDirection};

use crate::api::APIConnection;

/// Stores the current state of the application, and the connection
/// to the API used to update that data.
pub struct AppModel {
    api_conn: APIConnection,

    automation_id: i64,
    current_index: i64,
    pub automations: Vec<Automation>,
    pub triggers: Vec<AutomationTrigger>,
    pub actions: Vec<AutomationAction>,
}

impl AppModel {
    /// Constructs a new app model with default values, given a
    /// connection to the API.
    pub fn new(api_conn: APIConnection) -> Self {
        return Self {
            api_conn: api_conn,
            automation_id: -1,
            current_index: -1,
            automations: Vec::new(),
            triggers: Vec::new(),
            actions: Vec::new()
        };
    }

    /// Sets the currently selected automation ID to a different ID.
    pub fn set_current_automation_id(&mut self, automation_id: i64) {
        self.automation_id = automation_id;
    }

    /// Gets the currently selected automation's index in the
    /// 'automations' list.
    pub fn get_current_automation_index(&self) -> i64 {
        return self.current_index;
    }

    /// Sets the index of the current automation in the 'automations'
    /// list.
    pub fn set_current_automation_index(&mut self, index: i64) {
        self.current_index = index;
    }

    /// Updates the list of automations, retrieving them from the
    /// daemon.
    pub async fn update_automations_list(&mut self) {
        let automations_result = self.api_conn.get_automations().await;

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

    /// Adds a new automation, making the appropriate call to the
    /// daemon. Updates the model's own automations, triggers, and
    /// actions list too to select that new automation.
    pub async fn add_new_automation(&mut self) {
        let new_id_result = self.api_conn.add_automation("New Automation".to_string()).await;

        let new_id = match new_id_result {
            Ok(new_id) => new_id,
            Err(_) => {
                println!("Error creating new automation");
                std::process::exit(1);
            }
        };

        self.automation_id = new_id;

        self.update_automations_list().await;
        self.update_automation_index();

        self.update_triggers_list().await;
        self.update_actions_list().await;
    }

    /// Updates the index of the currently selected automation
    /// by searching for it by ID. If not found, the index is
    /// set to -1.
    pub fn update_automation_index(&mut self) {
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

    /// Updates the list of triggers by performing the appropriate
    /// API calls.
    pub async fn update_triggers_list(&mut self) {
        let triggers_result = self.api_conn.get_automation_triggers(self.automation_id).await;

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

    /// Updates the list of actions by performing the appropriate
    /// API calls.
    pub async fn update_actions_list(&mut self) {
        let actions_result = self.api_conn.get_automation_actions(self.automation_id).await;

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

    /// Sends any changes to the current automation back to the
    /// daemon to be stored in the DB.
    pub async fn sync_automation_changes(&mut self) {
        let _  = self.api_conn.update_automation(self.automations[self.current_index as usize].clone()).await;
    }

    /// Deletes the currently selected automation.
    pub async fn delete_automation(&mut self) {
        let res = self.api_conn.delete_automation(self.automation_id).await;

        if res.is_ok() {
            self.automation_id = -1;
            self.current_index = -1;
        }
    }

    /// Manually ru'/ns the currently selected automation.
    pub async fn run_automation(&mut self) {
        let _ = self.api_conn.run_automation(self.automation_id, true).await;
    }

    /// Adds a trigger to the current automation of the given
    /// type and with the provided JSON details.
    pub async fn add_trigger(&mut self, trig_type: i64, details: String) {
        let _ = self.api_conn.add_trigger(self.automation_id, trig_type, details).await;
    }

    /// Updates a trigger, provided its ID, with a new JSON
    /// string of details.
    pub async fn update_trigger(&mut self, trig_id: i64, new_details: String) {
        let _ = self.api_conn.update_trigger(trig_id, new_details).await;
    }

    /// Deletes a trigger, given its ID.
    pub async fn delete_trigger(&mut self, trig_id: i64) {
        let _ = self.api_conn.delete_trigger(trig_id).await;
    }

    /// Adds an action to the current automation, given its
    /// type and its details.
    pub async fn add_action(&mut self, action_type: i64, details: String) {
        let _ = self.api_conn.add_action(self.automation_id, action_type, details).await;
    }

    /// Updates an action, given its ID and its new details.
    pub async fn update_action(&mut self, action_id: i64, new_details: String) {
        let _ = self.api_conn.update_action(action_id, new_details).await;
    }

    /// Moves an action by 1 space, given its ID and the
    /// direction to move it in.
    pub async fn move_action(&mut self, action_id: i64, direction: MoveDirection) {
        let _ = self.api_conn.move_action(action_id, direction).await;
    }

    /// Deletes an action, given its ID.
    pub async fn delete_action(&mut self, action_id: i64) {
        let _ = self.api_conn.delete_action(action_id).await;
    }
}
