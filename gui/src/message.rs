use sigroute_common::MoveDirection;

#[derive(PartialEq, Clone)]
pub enum UIEvent {
    AddedAutomation,
    ChangedAutomation(i64), // The ID of the automation changed to
    UpdatedAutomationName(String), // What the new name of the automation is
    UpdatedAutomationActivity(bool), // Whether or not the automation is active
    AddedTrigger(i64, String), // The type of the trigger, and the trigger details
    UpdatedTrigger(i64, String), // The ID of the trigger, and the new trigger details
    DeletedTrigger(i64), // The ID of the trigger
    AddedAction(i64, String), // The type of the action, and the action details
    UpdatedAction(i64, String), // The ID of the action, and the new action details
    MoveAction(i64, MoveDirection), // The ID of the action, and the direction it is requested to move in
    DeletedAction(i64) // The ID of the action
}

#[derive(PartialEq, Clone, Copy)]
pub enum ModelUpdate {
    AutomationListUpdate,
    AutomationUpdate,
}
