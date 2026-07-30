use serde::{Deserialize, Serialize};
use zvariant::Type;
use zbus::DBusError;

/* === API Errors === */

#[derive(Serialize, Deserialize, DBusError, Debug)]
pub enum APIError {
    DBAccessError,
}

/* === Triggers === */

pub const T_TIME: i64 = 1;
pub const T_NETWORK_CONNECTED_TO: i64 = 2;
pub const T_NETWORK_DISCONNECTED_FROM: i64 = 3;
pub const T_POWER_CONNECTED: i64 = 4;
pub const T_POWER_DISCONNECTED: i64 = 5;
pub const T_USER_LOGIN: i64 = 6;

pub const TRIGGER_MAX: i64 = 6;

pub fn trigger_to_name(x: i64) -> String {
    match x {
        T_TIME => "Time-based".to_string(),
        T_NETWORK_CONNECTED_TO => "Network Connected".to_string(),
        T_NETWORK_DISCONNECTED_FROM => "Network Disconnected".to_string(),
        T_POWER_CONNECTED => "Power Connected".to_string(),
        T_POWER_DISCONNECTED => "Power Disconnected".to_string(),
        T_USER_LOGIN => "User Login".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn trigger_to_icon_name(x: i64) -> String {
    match x {
        T_TIME => "preferences-system-time-symbolic".to_string(),
        T_NETWORK_CONNECTED_TO => "network-workgroup-symbolic".to_string(),
        T_NETWORK_DISCONNECTED_FROM => "network-wired-disconnected-symbolic".to_string(),
        T_POWER_CONNECTED => "ac-adapter-symbolic".to_string(),
        T_POWER_DISCONNECTED => "battery-missing-symbolic".to_string(),
        T_USER_LOGIN => "avatar-default-symbolic".to_string(),
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

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
pub struct Automation {
    pub id: i64,
    pub name: String,
    pub active: bool,
}
