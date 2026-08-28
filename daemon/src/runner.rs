//! This module provides functionality to run actions
//! in the daemon, given as a list of AutomationActions.

use std::{collections::HashMap, process::Command};

use serde_json::{Map, Value};
use sigroute_common::{A_COMMAND, A_NOTIFICATION, AutomationAction};
use zbus::blocking::{Connection, Proxy};

/// Runs actions sequentially.
pub struct ActionsRunner {
    pub actions: Vec<AutomationAction>,
    pub current: i64
}

impl ActionsRunner {
    /// Constructs a new ActionRunner from the provided list
    /// of actions.
    pub fn new(actions: Vec<AutomationAction>) -> Self {
        Self {
            actions: actions,
            current: 0
        }
    }

    /// Runs the next action in the list of actions the struct
    /// was initialised with. Returns whether or not the action
    /// was run successfully.
    pub fn step(&mut self) -> bool {
        if !self.has_next() {
            return false;
        }

        // Getting the next action to run and its options
        let action = &self.actions[self.current as usize];
        let jsonified: Map<String, Value> = serde_json::from_str(&action.details).unwrap_or_default();

        match action.action_type {
            A_COMMAND => {
                // If this is a command, we extract the entered string from the string picker
                if let Some(command_options) = jsonified.get("command") {
                    if let Some(c) = command_options.get("string") {
                        let command_string = c.as_str().unwrap_or("").to_string();
                        
                        // Getting the command and the arguments (rudimentary)
                        let mut parts = command_string.split_whitespace();
                        let command_name = parts.next().unwrap_or("");

                        // Running the command
                        let _ = Command::new(command_name)
                            .args(parts)
                            .output();
                    }
                }
            }

            A_NOTIFICATION => {
                // If this is a notification, we extract the entered string from the string picker
                if let Some(notif_options) = jsonified.get("contents") {
                    if let Some(c) = notif_options.get("string") {
                        let contents = c.as_str().unwrap_or("").to_string();

                        // Sending the notification if the contents aren't empty
                        if contents != "" {
                            send_notification("Notification", &contents);
                        }
                    }
                }
            }

            _ => {
                // Any other action should fail to run as it is not implemented
                return false;
            }
        }

        self.current += 1;

        return true;
    }

    /// Returns whether or not this runner has any more actions
    /// to run.
    pub fn has_next(&self) -> bool {
        return self.actions.len() > self.current as usize;
    }

    /// Runs all of the actions, regardless of whether or not
    /// previous actions failed.
    pub fn run_all(&mut self) {
        for _ in self.current..self.actions.len() as i64 {
            self.step();
        }
    }
}

/// Sends a notification via the D-Bus interface with the given
/// title and contents.
pub fn send_notification(title: &str, contents: &str) {
    let connection_result = Connection::session();

    if connection_result.is_err() {
        return;
    }

    let connection = connection_result.unwrap();

    // Using org.freedesktop.Notifications.Notify to send the notification
    let proxy_result = Proxy::new(
        &connection,
        "org.freedesktop.Notifications",
        "/org/freedesktop/Notifications",
        "org.freedesktop.Notifications"
    );

    if proxy_result.is_err() {
        return;
    }

    let proxy = proxy_result.unwrap();

    // Calls the "Notify" method 
    let _: Result<u32, zbus::Error> = proxy.call(
        "Notify",
        &(
            "Sigroute", // Application Name
            0 as u32,   // ID of older notification to replace
            "dialog-information", // icon
            title,      // Notification title
            contents,   // Notification contents
            Vec::<&str>::new(), // Action buttons to display
            HashMap::<&str, zbus::zvariant::Value>::new(), // Hints (for extra styling)
            -1 as i32   // Timeout in ms (-1 = default)
        )
    );
}
