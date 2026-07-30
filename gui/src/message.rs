#[derive(PartialEq, Clone)]
pub enum UIEvent {
    AddedAutomation,
    ChangedAutomation(i64), // The ID of the automation changed to
    UpdateAutomationName(String),
}

#[derive(PartialEq, Clone, Copy)]
pub enum ModelUpdate {
    AutomationListUpdate,
    AutomationUpdate,
}
