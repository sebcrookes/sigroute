use serde::Serialize;
use zvariant::Type;

#[derive(Serialize, Type)]
pub enum AutomationTrigger {
    TimeBased(u64),
}

#[derive(Serialize, Type)]
pub enum AutomationAction {
    SendNotification(String),
}

#[derive(Serialize, Type)]
pub struct Automation {
    pub trigger: AutomationTrigger,
    pub actions: Vec<AutomationAction>,
}