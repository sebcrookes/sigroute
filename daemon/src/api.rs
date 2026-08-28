//! This module provides the ability for the GUI application
//! to interact with the daemon over the API.

use std::path::PathBuf;
use sigroute_common::A_NOTIFICATION;
use sigroute_common::APIError;
use sigroute_common::APIError::DBAccessError;
use sigroute_common::Automation;
use sigroute_common::AutomationAction;
use sigroute_common::AutomationTrigger;
use sigroute_common::MoveDirection;
use zbus::interface;

use crate::db;
use crate::runner::ActionsRunner;

/// Provides a way for clients to make requests to the
/// deamon. Holds information required to service client
/// requests.
pub struct AutomationAPI {
    db_path: PathBuf,
}

impl AutomationAPI {
    /// Constructs a new API struct.
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            db_path,
        }
    }
}

#[interface(name = "uk.co.sebcrookes.Sigroute")]
impl AutomationAPI {
    /// Returns the version of the daemon as a string.
    fn get_version(&self) -> String {
        return env!("CARGO_PKG_VERSION").to_string();
    }
    
    /// Returns a list of all of the automations registered with
    /// the daemon.
    fn get_automations(&self) -> Result<Vec<Automation>, APIError> {
        let result = db::get_all_automations(&self.db_path);

        match result {
            Ok(automations) => Ok(automations),
            Err(_) => Err(DBAccessError),
        }
    }

    /// Returns a list of all of the automation triggers for the
    /// automation with the provided ID.
    fn get_automation_triggers(&self, automation_id: i64) -> Result<Vec<AutomationTrigger>, APIError> {
        let result = db::get_triggers_for(&self.db_path, automation_id);

        match result {
            Ok(triggers) => Ok(triggers),
            Err(_) => Err(DBAccessError),
        }
    }

    /// Returns a list of all of the automation actions for the
    /// automation with the provided ID.
    fn get_automation_actions(&self, automation_id: i64) -> Result<Vec<AutomationAction>, APIError> {
        let result = db::get_automation_actions(&self.db_path, automation_id);

        match result {
            Ok(actions) => Ok(actions),
            Err(_) => Err(DBAccessError),
        }
    }

    /// Adds a new automation to the list of automations with
    /// the provided name.
    fn add_automation(&self, automation_name: String) -> Result<i64, APIError> {
        let result = db::add_automation(&self.db_path, automation_name);

        match result {
            Ok(automation_id) => Ok(automation_id),
            Err(_) => Err(DBAccessError),
        }
    }

    /// Updates the automation stored with the daemon with the
    /// new values set in the provided automation.
    fn update_automation(&self, automation: Automation) -> Result<(), APIError> {
        let result = db::update_automation(&self.db_path, automation);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DBAccessError),
        }
    }

    /// Deletes the automation with the provided ID.
    fn delete_automation(&self, automation_id: i64) -> Result<(), APIError> {
        let result = db::delete_automation(&self.db_path, automation_id);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DBAccessError),
        }
    }

    /// Runs the automation with the provided ID.
    fn run_automation(&self, automation_id: i64, is_manual: bool) -> Result<(), APIError> {
        let automation_result = db::get_automation(&self.db_path,automation_id);
        if automation_result.is_err() {
            return Err(DBAccessError);
        }

        let automation = automation_result.unwrap();

        let mut actions = self.get_automation_actions(automation_id)?;
        if is_manual {
            actions.push(AutomationAction {
                id: -1,
                action_type: A_NOTIFICATION,
                details: format!("{{\"contents\":{{\"string\":\"Successfully manually ran automation \\\"{}\\\"\"}}}}", automation.name)
            });
        }

        let mut runner = ActionsRunner::new(actions);
        runner.run_all();

        Ok(())
    }

    /// Adds a trigger with the given type and details to the
    /// automation with the provided ID.
    fn add_trigger(&self, automation_id: i64, trig_type: i64, details: String) -> Result<(), APIError> {
        let result = db::add_trigger(&self.db_path, automation_id, trig_type, details);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DBAccessError),
        }
    }

    /// Updates the trigger with the provided ID with the given
    /// new details.
    fn update_trigger(&self, trigger_id: i64, new_details: String) -> Result<(), APIError> {
        let result = db::update_trigger(&self.db_path, trigger_id, new_details);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DBAccessError),
        }
    }

    /// Deletes the trigger with the provided ID.
    fn delete_trigger(&self, trigger_id: i64) -> Result<(), APIError> {
        let result = db::delete_trigger(&self.db_path, trigger_id);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DBAccessError),
        }
    }

    /// Adds an action with the given type and details to the
    /// automation with the provided ID.
    fn add_action(&self, automation_id: i64, action_type: i64, details: String) -> Result<(), APIError> {
        let result = db::add_action(&self.db_path, automation_id, action_type, details);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DBAccessError),
        }
    }

    /// Updates the action with the given ID with the provided
    /// new details.
    fn update_action(&self, action_id: i64, new_details: String) -> Result<(), APIError> {
        let result = db::update_action(&self.db_path, action_id, new_details);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DBAccessError),
        }
    }

    /// Moves the action with the given ID up or down in the list
    /// one place in the provided direction.
    fn move_action(&self, action_id: i64, direction: MoveDirection) -> Result<(), APIError> {
        let result = db::move_action(&self.db_path, action_id, direction);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DBAccessError),
        }
    }

    /// Deletes the action with the provided ID.
    fn delete_action(&self, action_id: i64) -> Result<(), APIError> {
        let result = db::delete_action(&self.db_path, action_id);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DBAccessError),
        }
    }
}
