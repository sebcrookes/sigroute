use async_channel::Sender;
use gtk4::{gio::Cancellable, glib};
use libadwaita::{AlertDialog, ApplicationWindow, prelude::{AlertDialogExt, AlertDialogExtManual}};

use crate::message::UIEvent::{self, DeletedTrigger};

#[derive(Clone)]
pub struct DeleteMenu {}

impl DeleteMenu {
    pub fn new(sender: &Sender<UIEvent>, window: &ApplicationWindow, trigger_id: i64) -> Self {
        let menu = AlertDialog::builder()
            .heading("Delete Trigger")
            .body("Are you sure you want to delete this trigger?")
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
                    s.send(DeletedTrigger(trigger_id)).await.unwrap();
                });
            }
        });
        
        let model = Self {};
        
        model
    }
}
