#[derive(PartialEq, Clone)]
pub enum UIEvent {
    AddedAutomation,
    ChangedAutomation(i64), // The ID of the automation changed to
    UpdatedAutomationName(String), // What the new name of the automation is
    UpdatedAutomationActivity(bool), // Whether or not the automation is active
    AddedTrigger(i64, String), // The type of the trigger, and the trigger details
    AddedAction(i64, String), // The type of the action, and the action details
}

#[derive(PartialEq, Clone, Copy)]
pub enum ModelUpdate {
    AutomationListUpdate,
    AutomationUpdate,
}
