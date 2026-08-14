use async_channel::Sender;
use gtk4::{gio::Cancellable, glib};
use libadwaita::{AlertDialog, ApplicationWindow, prelude::{AlertDialogExt, AlertDialogExtManual}};

use crate::message::UIEvent::{self, DeletedTrigger};

pub enum DeleteType {
    Trigger,
    Action
}

#[derive(Clone)]
pub struct DeleteMenu {}

impl DeleteMenu {
    pub fn new(sender: &Sender<UIEvent>, window: &ApplicationWindow, delete_type: DeleteType, id: i64) -> Self {
        let heading = match delete_type {
            DeleteType::Trigger => "Delete Trigger",
            DeleteType::Action => "Delete Action"
        };

        let body = match delete_type {
            DeleteType::Trigger => "Are you sure you want to delete this trigger?",
            DeleteType::Action => "Are you sure you want to delete this action?"
        };

        let menu = AlertDialog::builder()
            .heading(heading)
            .body(body)
            .build();

        menu.add_responses(&[
            ("cancel", "Cancel"),
            ("delete", "Delete")
        ]);
        menu.set_response_appearance("delete", libadwaita::ResponseAppearance::Destructive);

        /* Showing the dialog */
        let s = sender.clone();
        menu.choose(Some(window), None::<&Cancellable>, move |option| {
            if option == "delete" {
                let s = s.clone();
                glib::spawn_future_local(async move {
                    match delete_type {
                        DeleteType::Trigger => {
                            s.send(DeletedTrigger(id)).await.unwrap();
                        }
                        DeleteType::Action => {}
                    };
                });
            }
        });
        
        let model = Self {};
        
        model
    }
}
