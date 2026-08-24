//! This module provides enums allowing for messages to be sent between
//! the model, views, and controller. UIEvent allows views to notify the 
//! controller of actions which the user has performed, and ModelUpdate
//! allows the views to be notified of model updates.

use sigroute_common::MoveDirection;

/// Represents an action which has been performed by the user in the UI,
/// and is sent by views to notify the controller of the update.
#[derive(PartialEq, Clone)]
pub enum UIEvent {
    /// A request to add an automation has been made.
    AddedAutomation,

    /// A request to change the selected automation to the given automation ID
    /// has been made.
    ChangedAutomation(i64),

    /// A request to change the current automation's name to the provided name has
    /// been made.
    UpdatedAutomationName(String),

    /// A request to update whether or not an automation is active (to the provided
    /// value) has been made.
    UpdatedAutomationActivity(bool),

    /// A request to delete the current automation has been made.
    DeletedAutomation(),

    /// A request to manually run the current automation has been made.
    RanAutomation(),

    /// A request to add a trigger of the provided type and with the provided
    /// details, has been made.
    AddedTrigger(i64, String),

    /// A request to update the trigger with the provided ID to the provided
    /// JSON details has been made.
    UpdatedTrigger(i64, String),

    /// A request to delete the trigger with the provided ID has been made.
    DeletedTrigger(i64),

    /// A request to add an action with the provided type and details has been
    /// made.
    AddedAction(i64, String),

    /// A request to update an action with the provided ID to the provided JSON
    /// details has been made.
    UpdatedAction(i64, String),

    /// A request to move an action with the provided ID one space in the provided
    /// direction has been made.
    MoveAction(i64, MoveDirection),

    /// A request to delete the action with the provided ID has been made.
    DeletedAction(i64)
}

/// Represents an update which has occurred with the model, and is sent
/// to views to update the content they are dispaying.
#[derive(PartialEq, Clone, Copy)]
pub enum ModelUpdate {
    /// The model has been changed such that the automation list must be redrawn.
    AutomationListUpdate,

    /// The model has changed such that the current automation must be redrawn.
    AutomationUpdate
}
