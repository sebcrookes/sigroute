use serde::Serialize;
use zvariant::Type;

/* === Triggers === */

pub const TRIGGER_TIME: i64 = 1;

pub fn trigger_to_name(x: i64) -> String {
    match x {
        TRIGGER_TIME => "Time-based".to_string(),
        _ => "Unknown".to_string()
    }
}

#[derive(Serialize, Type)]
pub struct AutomationTrigger {
    pub id: i64,
    pub json: String,
}

/* === Actions === */

pub const ACTION_COMMAND: i64 = 1;
pub const ACTION_NOTIFICATION: i64 = 2;

pub fn action_to_name(x: i64) -> String {
    match x {
        ACTION_COMMAND => "Run command".to_string(),
        ACTION_NOTIFICATION => "Send notification".to_string(),
        _ => "Unknown".to_string()
    }
}

#[derive(Serialize, Type)]
pub struct AutomationAction {
    pub id: i64,
    pub json: String,
}

#[derive(Serialize, Type, Debug)]
pub struct Automation {
    pub id: i64,
    pub name: String,
}
