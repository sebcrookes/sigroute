use serde::{Deserialize, Serialize};
use zvariant::Type;
use zbus::DBusError;

/* === API Errors === */

#[derive(Serialize, Deserialize, DBusError, Debug)]
pub enum APIError {
    DBAccessError,
}

/* === Triggers === */

pub const TRIGGER_TIME: i64 = 1;

pub fn trigger_to_name(x: i64) -> String {
    match x {
        TRIGGER_TIME => "Time-based".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn trigger_to_icon_name(x: i64) -> String {
    match x {
        TRIGGER_TIME => "preferences-system-time-symbolic".to_string(),
        _ => "value-decrease-symbolic".to_string(),
    }
}

#[derive(Serialize, Deserialize, Type)]
pub struct AutomationTrigger {
    pub id: i64,
    pub trig_type: i64,
    pub details: String,
}

/* === Actions === */

pub const ACTION_COMMAND: i64 = 1;
pub const ACTION_NOTIFICATION: i64 = 2;

pub fn action_to_name(x: i64) -> String {
    match x {
        ACTION_COMMAND => "Run command".to_string(),
        ACTION_NOTIFICATION => "Send notification".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn action_to_icon_name(x: i64) -> String {
    match x {
        ACTION_COMMAND => "utilities-terminal-symbolic".to_string(),
        ACTION_NOTIFICATION => "preferences-system-notifications-symbolic".to_string(),
        _ => "value-decrease-symbolic".to_string(),
    }
}

#[derive(Serialize, Deserialize, Type)]
pub struct AutomationAction {
    pub id: i64,
    pub action_type: i64,
    pub details: String,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct Automation {
    pub id: i64,
    pub name: String,
}
