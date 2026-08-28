//! This module allows for actions to be summarised into
//! human-readable strings.

use serde_json::{Map, Value};
use sigroute_common::{A_COMMAND, A_NOTIFICATION, AutomationAction};

/// Summarises the provided action into a format used for the subtitle in
/// the GUI.
pub fn summarise(a: &AutomationAction) -> String {
    match a.action_type {
        A_COMMAND => {
            let jsonified: Map<String, Value> = serde_json::from_str(&a.details).unwrap_or_default();

            // Getting the string from the string picker
            let summary_string = summarise_string_picker(&jsonified, "command");

            return format!("Run command: <span alpha='60%'>{summary_string}</span>");
        }
        A_NOTIFICATION => {
            let jsonified: Map<String, Value> = serde_json::from_str(&a.details).unwrap_or_default();

            // Getting the string from the string picker
            let summary_string = summarise_string_picker(&jsonified, "contents");

            return format!("Send notification: <span alpha='60%'>\"{summary_string}\"</span>");
        }
        _ => {
            return "".to_string();
        }
    }
}

/// Summarises the JSON provided by a string picker, given the root
/// JSON and the key at which it is stored.
fn summarise_string_picker(root_json: &Map<String, Value>, key: &str) -> String {
    let mut summary_string = "".to_string();
    let mut success = false;
    if let Some(picker) = root_json.get(key) {
        if let Some(s) = picker.get("string") {
            summary_string = s.as_str().unwrap_or("").to_string();
            success = true;
        }
    }

    if !success {
        summary_string = "[invalid string]".to_string();
    }

    return summary_string;
}
