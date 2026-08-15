use serde::{Deserialize, Serialize};
use zvariant::Type;
use zbus::DBusError;

/* === API Errors === */

#[derive(Serialize, Deserialize, DBusError, Debug)]
pub enum APIError {
    DBAccessError,
}

/* === Triggers === */

pub const T_REPEAT_EVERY: i64 = 1;
pub const T_CERTAIN_DAYS: i64 = 2;
pub const T_NETWORK_CONNECTED_TO: i64 = 3;
pub const T_NETWORK_DISCONNECTED_FROM: i64 = 4;
pub const T_POWER_CONNECTED: i64 = 5;
pub const T_POWER_DISCONNECTED: i64 = 6;
pub const T_USER_LOGIN: i64 = 7;

pub const TRIGGER_MAX: i64 = 2;

pub fn trigger_to_name(x: i64) -> String {
    match x {
        T_REPEAT_EVERY => "Repeat every".to_string(),
        T_CERTAIN_DAYS => "Repeat on certain days".to_string(),
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
        T_REPEAT_EVERY => "preferences-system-time-symbolic".to_string(),
        T_CERTAIN_DAYS => "x-office-calendar-symbolic".to_string(),
        T_NETWORK_CONNECTED_TO => "network-workgroup-symbolic".to_string(),
        T_NETWORK_DISCONNECTED_FROM => "network-wired-disconnected-symbolic".to_string(),
        T_POWER_CONNECTED => "ac-adapter-symbolic".to_string(),
        T_POWER_DISCONNECTED => "battery-missing-symbolic".to_string(),
        T_USER_LOGIN => "avatar-default-symbolic".to_string(),
        _ => "value-decrease-symbolic".to_string(),
    }
}

#[derive(Clone, Copy)]
pub enum OptionType {
    DateTime,
    Frequency,
    Days,
    Time,
    String,
    Unknown,
}

pub struct OptionDetails {
    pub opt_type: OptionType,
    pub title: String,
    pub subtitle: String,
    pub mandatory: bool,
    pub json_name: String
}

pub fn trigger_get_option_details(x: i64) -> Vec<OptionDetails> {
    match x {
        T_REPEAT_EVERY => Vec::from([
            OptionDetails {
                opt_type: OptionType::Frequency,
                title: "Frequency".to_string(),
                subtitle: "Click to edit how often it triggers".to_string(),
                mandatory: true,
                json_name: "frequency".to_string(),
            },
            OptionDetails {
                opt_type: OptionType::DateTime,
                title: "Starting at".to_string(),
                subtitle: "Click to edit the start date and time".to_string(),
                mandatory: true,
                json_name: "start-date".to_string(),
            }
        ]),
        T_CERTAIN_DAYS => Vec::from([
            OptionDetails {
                opt_type: OptionType::Days,
                title: "Days of the week".to_string(),
                subtitle: "Click to edit the days of the week this option triggers on".to_string(),
                mandatory: true,
                json_name: "days-active".to_string(),
            },
            OptionDetails {
                opt_type: OptionType::Time,
                title: "Trigger at".to_string(),
                subtitle: "Click to edit the time of day the option will trigger at".to_string(),
                mandatory: true,
                json_name: "time".to_string(),
            }
        ]),
        _ => Vec::from([
            OptionDetails {
                opt_type: OptionType::Unknown,
                title: "Unknown Option".to_string(),
                subtitle: "".to_string(),
                mandatory: false,
                json_name: "unknown".to_string(),
            }
        ])
    }
}

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct AutomationTrigger {
    pub id: i64,
    pub trig_type: i64,
    pub details: String,
}

/* === Actions === */

pub const A_COMMAND: i64 = 1;
pub const A_NOTIFICATION: i64 = 2;

pub const ACTION_MAX: i64 = 2;

pub fn action_to_name(x: i64) -> String {
    match x {
        A_COMMAND => "Run command".to_string(),
        A_NOTIFICATION => "Send notification".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn action_to_icon_name(x: i64) -> String {
    match x {
        A_COMMAND => "utilities-terminal-symbolic".to_string(),
        A_NOTIFICATION => "preferences-system-notifications-symbolic".to_string(),
        _ => "value-decrease-symbolic".to_string(),
    }
}

pub fn action_get_option_details(x: i64) -> Vec<OptionDetails> {
    match x {
        A_COMMAND => Vec::from([
            OptionDetails {
                opt_type: OptionType::String,
                title: "Command".to_string(),
                subtitle: "Click to edit the command being run".to_string(),
                mandatory: true,
                json_name: "command".to_string(),
            }
        ]),
        A_NOTIFICATION => Vec::from([
            OptionDetails {
                opt_type: OptionType::String,
                title: "Notification Contents".to_string(),
                subtitle: "Click to edit the notification contents".to_string(),
                mandatory: true,
                json_name: "contents".to_string(),
            }
        ]),
        _ => Vec::from([
            OptionDetails {
                opt_type: OptionType::Unknown,
                title: "Unknown Option".to_string(),
                subtitle: "".to_string(),
                mandatory: false,
                json_name: "unknown".to_string(),
            }
        ])
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

#[derive(Serialize, Deserialize, Type, Clone, Copy, PartialEq)]
pub enum MoveDirection {
    Up,
    Down,
}
