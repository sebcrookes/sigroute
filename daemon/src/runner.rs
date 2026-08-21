use std::{collections::HashMap, process::Command};

use serde_json::{Map, Value};
use sigroute_common::{A_COMMAND, A_NOTIFICATION, AutomationAction};
use zbus::blocking::{Connection, Proxy};

pub struct ActionsRunner {
    pub actions: Vec<AutomationAction>,
    pub current: i64
}

impl ActionsRunner {
    pub fn new(actions: Vec<AutomationAction>) -> Self {

        Self {
            actions: actions,
            current: 0
        }
    }

    pub fn step(&mut self) {
        let action = &self.actions[self.current as usize];

        let jsonified: Map<String, Value> = serde_json::from_str(&action.details).unwrap_or_default();

        match action.action_type {
            A_COMMAND => {
                if let Some(command_options) = jsonified.get("command") {
                    if let Some(c) = command_options.get("string") {
                        let command_string = c.as_str().unwrap_or("").to_string();
                        
                        let mut parts = command_string.split_whitespace();

                        let command_name = parts.next().unwrap_or("");

                        let _ = Command::new(command_name)
                            .args(parts)
                            .output();
                    }
                }
            }

            A_NOTIFICATION => {
                if let Some(notif_options) = jsonified.get("contents") {
                    if let Some(c) = notif_options.get("string") {
                        let contents = c.as_str().unwrap_or("").to_string();

                        if contents != "" {
                            send_notification("Sigroute Notification", &contents);
                        }
                    }
                }
            }

            _ => {
                
            }
        }

        self.current += 1;
    }

    pub fn run_all(&mut self) {
        for _ in self.current..self.actions.len() as i64 {
            self.step();
        }
    }
}

pub fn send_notification(title: &str, contents: &str) {
    let connection_result = Connection::session();

    if connection_result.is_err() {
        return;
    }

    let connection = connection_result.unwrap();

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

    let _: Result<u32, zbus::Error> = proxy.call(
        "Notify",
        &(
            "sigrouted",
            0 as u32,
            "dialog-information",
            title,
            contents,
            Vec::<&str>::new(),
            HashMap::<&str, zbus::zvariant::Value>::new(),
            -1 as i32
        )
    );
}
