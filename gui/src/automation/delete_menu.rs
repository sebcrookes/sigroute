use async_channel::Sender;
use gtk4::{gio::Cancellable, glib};
use libadwaita::{AlertDialog, ApplicationWindow, prelude::{AlertDialogExt, AlertDialogExtManual}};

use crate::message::UIEvent::{self, DeletedAction, DeletedAutomation, DeletedTrigger};

#[derive(PartialEq)]
pub enum DeleteType {
    Automation,
    Trigger,
    Action
}

#[derive(Clone)]
pub struct DeleteMenu {}

impl DeleteMenu {
    pub fn new(sender: &Sender<UIEvent>, window: &ApplicationWindow, delete_type: DeleteType, id: Option<i64>) -> Self {
        let heading = match delete_type {
            DeleteType::Automation => "Delete Automation",
            DeleteType::Trigger => "Delete Trigger",
            DeleteType::Action => "Delete Action"
        };

        if [DeleteType::Trigger, DeleteType::Action].contains(&delete_type) && id.is_none() {
            panic!("DeleteMenu expects an id for the provided DeleteType");
        }

        let body = match delete_type {
            DeleteType::Automation => "Are you sure you want to delete this automation?",
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
                        DeleteType::Automation => {
                            s.send(DeletedAutomation()).await.unwrap();
                        }
                        DeleteType::Trigger => {
                            // .unwrap is safe as we have already checked that it is a value
                            s.send(DeletedTrigger(id.unwrap())).await.unwrap();
                        }
                        DeleteType::Action => {
                            // .unwrap is safe as we have already checked that it is a value
                            s.send(DeletedAction(id.unwrap())).await.unwrap();
                        }
                    };
                });
            }
        });
        
        let model = Self {};
        
        model
    }
}
