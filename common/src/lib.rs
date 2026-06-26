use serde::Serialize;
use zvariant::Type;

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

#[derive(Serialize, Type)]
pub enum AutomationAction {
    SendNotification(String),
}

#[derive(Serialize, Type)]
pub struct Automation {
    pub triggers: Vec<AutomationTrigger>,
    pub actions: Vec<AutomationAction>,
}
