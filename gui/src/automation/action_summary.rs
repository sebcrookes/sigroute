use serde_json::{Map, Value};
use sigroute_common::{A_COMMAND, A_NOTIFICATION, AutomationAction};

/// Summarises the provided action into a format used for the subtitle in
/// the GUI.
pub fn summarise(t: &AutomationAction) -> String {
    match t.action_type {
        A_COMMAND => {
            let jsonified: Map<String, Value> = serde_json::from_str(&t.details).unwrap_or_default();

            // Getting the string from the string picker
            let summary_string = summarise_string_picker(&jsonified, "command");

            return summary_string;
        }
        A_NOTIFICATION => {
            let jsonified: Map<String, Value> = serde_json::from_str(&t.details).unwrap_or_default();

            // Getting the string from the string picker
            let summary_string = summarise_string_picker(&jsonified, "contents");

            return format!("Notification to send: \"{summary_string}\"");
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
    if let Some(frequency) = root_json.get(key) {
        if let Some(s) = frequency.get("string") {
            summary_string = s.as_str().unwrap_or("").to_string();
            success = true;
        }
    }

    if !success {
        summary_string = "[invalid string]".to_string();
    }

    return summary_string;
}
