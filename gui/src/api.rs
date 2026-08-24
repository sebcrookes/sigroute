//! This module provides the ability for the GUI application
//! to interact with the daemon over the API.

use sigroute_common::Automation;
use sigroute_common::AutomationAction;
use sigroute_common::AutomationTrigger;
use sigroute_common::MoveDirection;
use zbus::Connection;
use zbus::proxy;

#[proxy(
    interface = "uk.co.sebcrookes.Sigroute",
    default_service = "uk.co.sebcrookes.Sigroute",
    default_path = "/uk/co/sebcrookes/Sigroute",
    gen_blocking = true
)]
trait AutomationAPI {
    fn get_version(&self) -> zbus::Result<String>;
    fn get_automations(&self) -> zbus::Result<Vec<Automation>>;
    fn get_automation_triggers(&self, automation_id: i64) -> zbus::Result<Vec<AutomationTrigger>>;
    fn get_automation_actions(&self, automation_id: i64) -> zbus::Result<Vec<AutomationAction>>;
    fn add_automation(&self, automation_name: String) -> zbus::Result<i64>;
    fn update_automation(&self, automation: Automation) -> zbus::Result<()>;
    fn delete_automation(&self, automation_id: i64) -> zbus::Result<()>;
    fn run_automation(&self, automation_id: i64, is_manual: bool) -> zbus::Result<()>;
    fn add_trigger(&self, automation_id: i64, trig_type: i64, details: String) -> zbus::Result<()>;
    fn update_trigger(&self, trigger_id: i64, new_details: String) -> zbus::Result<()>;
    fn delete_trigger(&self, trigger_id: i64) -> zbus::Result<()>;
    fn add_action(&self, automation_id: i64, action_type: i64, details: String) -> zbus::Result<()>;
    fn update_action(&self, action_id: i64, new_details: String) -> zbus::Result<()>;
    fn move_action(&self, action_id: i64, direction: MoveDirection) -> zbus::Result<()>;
    fn delete_action(&self, action_id: i64) -> zbus::Result<()>;
}

/// Holds all the information needed to communicate over
/// the D-Bus API to the daemon.
pub struct APIConnection {
    _connection: Connection,
    proxy: AutomationAPIProxy<'static>,
}

/// Constructs a new D-Bus connection with the daemon.
/// Can return Err if there was an issue with initialising
/// the connection.
pub async fn open_connection() -> zbus::Result<APIConnection> {
    let connection = Connection::session().await?;
    let proxy = AutomationAPIProxy::new(&connection).await?;

    Ok(APIConnection { _connection: connection, proxy: proxy })
}

impl APIConnection {
    /// Gets the version of the daemon as a string.
    pub async fn get_version(&self) -> zbus::Result<String> {
        return self.proxy.get_version().await;
    }

    /// Gets a list of all of the automations registered with
    /// the daemon.
    pub async fn get_automations(&self) -> zbus::Result<Vec<Automation>> {
        return self.proxy.get_automations().await;
    }

    /// Gets a list of all of the automation triggers for the
    /// automation with the provided ID.
    pub async fn get_automation_triggers(&self, automation_id: i64) -> zbus::Result<Vec<AutomationTrigger>> {
        return self.proxy.get_automation_triggers(automation_id).await;
    }

    /// Gets a list of all of the automation actions for the
    /// automation with the provided ID.
    pub async fn get_automation_actions(&self, automation_id: i64) -> zbus::Result<Vec<AutomationAction>> {
        return self.proxy.get_automation_actions(automation_id).await;
    }

    /// Adds a new automation to the list of automations with
    /// the provided name.
    pub async fn add_automation(&self, automation_name: String) -> zbus::Result<i64> {
        return self.proxy.add_automation(automation_name).await;
    }

    /// Updates the automation stored with the daemon with the
    /// new values set in the provided automation.
    pub async fn update_automation(&self, automation: Automation) -> zbus::Result<()> {
        return self.proxy.update_automation(automation).await;
    }

    /// Deletes the automation with the provided ID.
    pub async fn delete_automation(&self, automation_id: i64) -> zbus::Result<()> {
        return self.proxy.delete_automation(automation_id).await;
    }

    /// Runs the automation with the provided ID.
    pub async fn run_automation(&self, automation_id: i64, is_manual: bool) -> zbus::Result<()> {
        return self.proxy.run_automation(automation_id, is_manual).await;
    }

    /// Adds a trigger with the given type and details to the
    /// automation with the provided ID.
    pub async fn add_trigger(&self, automation_id: i64, trig_type: i64, details: String) -> zbus::Result<()> {
        return self.proxy.add_trigger(automation_id, trig_type, details).await;
    }

    /// Updates the trigger with the provided ID with the given
    /// new details.
    pub async fn update_trigger(&self, trigger_id: i64, new_details: String) -> zbus::Result<()> {
        return self.proxy.update_trigger(trigger_id, new_details).await;
    }

    /// Deletes the trigger with the provided ID.
    pub async fn delete_trigger(&self, trigger_id: i64) -> zbus::Result<()> {
        return self.proxy.delete_trigger(trigger_id).await;
    }

    /// Adds an action with the given type and details to the
    /// automation with the provided ID.
    pub async fn add_action(&self, automation_id: i64, action_type: i64, details: String) -> zbus::Result<()> {
        return self.proxy.add_action(automation_id, action_type, details).await;
    }

    /// Updates the action with the given ID with the provided
    /// new details.
    pub async fn update_action(&self, action_id: i64, new_details: String) -> zbus::Result<()> {
        return self.proxy.update_action(action_id, new_details).await;
    }

    /// Moves the action with the given ID up or down in the list
    /// one place in the provided direction.
    pub async fn move_action(&self, action_id: i64, direction: MoveDirection) -> zbus::Result<()> {
        return self.proxy.move_action(action_id, direction).await;
    }

    /// Deletes the action with the provided ID.
    pub async fn delete_action(&self, action_id: i64) -> zbus::Result<()> {
        return self.proxy.delete_action(action_id).await;
    }
}
