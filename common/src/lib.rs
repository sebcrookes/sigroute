use serde::Serialize;
use zvariant::Type;

/* === Triggers === */

pub const TRIGGER_TIME: u32 = 1;

pub fn trigger_to_name(x: u32) -> String {
    match x {
        TRIGGER_TIME => "Time-based".to_string(),
        _ => "Unknown".to_string()
    }
}

#[derive(Serialize, Type)]
pub struct AutomationTrigger {
    pub id: u32,
    pub json: String,
}

/* === Actions === */

pub const ACTION_COMMAND: u32 = 1;
pub const ACTION_NOTIFICATION: u32 = 2;

pub fn action_to_name(x: u32) -> String {
    match x {
        ACTION_COMMAND => "Run command".to_string(),
        ACTION_NOTIFICATION => "Send notification".to_string(),
        _ => "Unknown".to_string()
    }
}

#[derive(Serialize, Type)]
pub struct AutomationAction {
    pub id: u32,
    pub json: String,
}

#[derive(Serialize, Type)]
pub struct Automation {
    pub triggers: Vec<AutomationTrigger>,
    pub actions: Vec<AutomationAction>,
}
