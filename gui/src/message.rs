#[derive(PartialEq, Clone, Copy)]
pub enum UIEvent {
    AddedAutomation,
    ChangedAutomation(i64), // The ID of the automation changed to
}

#[derive(PartialEq, Clone, Copy)]
pub enum ModelUpdate {
    AutomationListUpdate,
    AutomationUpdate,
}
