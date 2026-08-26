//! This module allows for triggers to be summarised into
//! human-readable strings.

use std::slice::Iter;

use serde_json::{Map, Value, json};
use sigroute_common::{AutomationTrigger, T_CERTAIN_DAYS, T_REPEAT_EVERY};

/// Summarises the provided trigger into a format used for the subtitle in
/// the GUI.
pub fn summarise(t: &AutomationTrigger) -> String {
    match t.trig_type {
        T_REPEAT_EVERY => {
            let jsonified: Map<String, Value> = serde_json::from_str(&t.details).unwrap_or_default();

            // Getting the string for the frequency
            let frequency_string = summarise_frequency_picker(&jsonified, "frequency");

            // Getting the string for the start date and time
            let (date_string, time_string) = summarise_datetime_picker(&jsonified, "start-date");

            return format!("Repeat every {}, starting on {} at {}", frequency_string, date_string, time_string);
        }
        T_CERTAIN_DAYS => {
            let jsonified: Map<String, Value> = serde_json::from_str(&t.details).unwrap_or_default();

            // Getting the string for the days the trigger is active
            let days_string = summarise_days_picker(&jsonified, "days-active");

            // Getting the string for the time
            let time_string = summarise_time_picker(&jsonified, "time");

            return format!("{}, at {}", days_string, time_string);
        }
        _ => {
            return "".to_string();
        }
    }
}

/// Summarises the JSON provided by a frequency picker, given the root
/// JSON and the key at which it is stored.
fn summarise_frequency_picker(root_json: &Map<String, Value>, key: &str) -> String {
    let mut frequency_string = "".to_string();
    if let Some(frequency) = root_json.get(key) {
        if let Some(y) = frequency.get("year") {
            frequency_string = get_time_unit_as_str(y, "year");
        } else if let Some(mo) = frequency.get("month") {
            frequency_string = get_time_unit_as_str(mo, "month");
        } else if let Some(d) = frequency.get("day") {
            frequency_string = get_time_unit_as_str(d, "day");
        } else {
            if let Some(h) = frequency.get("hour") {
                frequency_string = get_time_unit_as_str(h, "hour");
            }

            if let Some(m) = frequency.get("minute") {
                let minute_str = get_time_unit_as_str(m, "minute");
                frequency_string = concatenate_strs_with_comma(frequency_string, minute_str);
            }

            if let Some(s) = frequency.get("second") {
                let second_str = get_time_unit_as_str(s, "second");
                frequency_string = concatenate_strs_with_comma(frequency_string, second_str);
            }
        }
    }

    if frequency_string == "" {
        frequency_string = "[invalid frequency]".to_string();
    }

    return frequency_string;
}

/// Summarises the JSON provided by a datetime picker, given the root
/// JSON and the key at which it is stored.
fn summarise_datetime_picker(root_json: &Map<String, Value>, key: &str) -> (String, String) {
    let mut date_string = "".to_string();
    let mut time_string = "".to_string();

    if let Some(datetime) = root_json.get(key) {                
        let year = datetime.get("year").unwrap_or(&json!(-1)).as_i64().unwrap_or(-1);
        let month = datetime.get("month").unwrap_or(&json!(-1)).as_i64().unwrap_or(-1);
        let day = datetime.get("day").unwrap_or(&json!(-1)).as_i64().unwrap_or(-1);
        let hour = datetime.get("hour").unwrap_or(&json!(-1)).as_i64().unwrap_or(-1);
        let minute = datetime.get("minute").unwrap_or(&json!(-1)).as_i64().unwrap_or(-1);
        let second = datetime.get("second").unwrap_or(&json!(-1)).as_i64().unwrap_or(-1);

        date_string = format!("{:02}/{:02}/{}", day, month, year);
        if [year, month, day].iter().any(|x| *x == -1) {
            date_string = "[invalid date]".to_string();
        }

        time_string = format!("{:02}:{:02}:{:02}", hour, minute, second);
        if [hour, minute, second].iter().any(|x| *x == -1) {
            time_string = "[invalid time]".to_string();
        }
    }

    return (date_string, time_string);
}

/// Summarises the JSON provided by a days picker, given the root
/// JSON and the key at which it is stored.
fn summarise_days_picker(root_json: &Map<String, Value>, key: &str) -> String {
    let mut days_string = "".to_string();
    let day_names = ["Mondays", "Tuesdays", "Wednesdays", "Thursdays", "Fridays", "Saturdays", "Sundays"];
    
    if let Some(days) = root_json.get(key) {
        if let Some(days_obj) = days.as_object() {
            let mut enabled = [false; 7];

            for i in 1..=7 {
                if let Some(val) = days_obj.get(&format!("{}", i)) {
                    enabled[i - 1] = val.as_i64().unwrap_or(-1) == 1;
                }
            }

            let num_enabled = enabled.iter().filter(|e| **e == true).count();
            
            if num_enabled == 7 {
                days_string = "Every day".to_string();
            } else if num_enabled >= 5 {
                let disabled_names: Vec<&str> = day_names.iter().enumerate().filter(|(i, _)| !enabled[*i]).map(|(_, e)| *e).collect();
                days_string = format!("Every day, except {}", concatenate_list_with_and(disabled_names.iter()));
            } else {
                let enabled_names: Vec<&str> = day_names.iter().enumerate().filter(|(i, _)| enabled[*i]).map(|(_, e)| *e).collect();
                days_string = format!("On {}", concatenate_list_with_and(enabled_names.iter()));
            }
        }
    }

    return days_string;
}

/// Summarises the JSON provided by a time picker, given the root
/// JSON and the key at which it is stored.
fn summarise_time_picker(root_json: &Map<String, Value>, key: &str) -> String {
    let mut time_string = "".to_string();

    if let Some(datetime) = root_json.get(key) {                
        let hour = datetime.get("hour").unwrap_or(&json!(-1)).as_i64().unwrap_or(-1);
        let minute = datetime.get("minute").unwrap_or(&json!(-1)).as_i64().unwrap_or(-1);
        let second = datetime.get("second").unwrap_or(&json!(-1)).as_i64().unwrap_or(-1);

        time_string = format!("{:02}:{:02}:{:02}", hour, minute, second);
        if [hour, minute, second].iter().any(|x| *x == -1) {
            time_string = "[invalid time]".to_string();
        }
    }

    return time_string;
}

/// Appends a unit of time to a provided JSON value (expected as an i64).
/// 
/// # Errors
/// Returns -1 in place of the value if the value provided is not an i64.
fn get_time_unit_as_str(v: &Value, name: &str) -> String {
    let as_int = v.as_i64().unwrap_or(-1);
    if as_int == 1 {
        return format!("{} {}", as_int, name);
    } else {
        return format!("{} {}s", as_int, name);
    }
}

/// Concatenates two strings - if the first string is non-empty, it inserts
/// a comma and a space between the two. Otherwise, it returns the second string.
fn concatenate_strs_with_comma(first: String, second: String) -> String {
    if first == "" {
        return second;
    } else {
        return format!("{}, {}", first, second);
    }
}

/// Concatenates all items in a list with commas, with the final two items being
/// separated by "and".
fn concatenate_list_with_and(mut list: Iter<&str>) -> String {
    let mut result = "".to_string();
    
    let mut older = list.next();
    if let Some(s) = older {
        result = s.to_string();
    }

    let mut old = list.next();

    while let Some(s) = list.next() {
        older = old;
        if let Some(s) = older {
            result = format!("{}, {}", result, s);
        }

        old = Some(s);
    }

    if let Some(s) = old {
        result = format!("{} and {}", result, s);
    }

    return result;
}
