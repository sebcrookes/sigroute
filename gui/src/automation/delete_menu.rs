use async_channel::Sender;
use gtk4::{MessageDialog, gio::{self, Cancellable}, prelude::GtkWindowExt};
use libadwaita::{AlertDialog, ApplicationWindow, prelude::{AlertDialogExt, AlertDialogExtManual}};

use crate::message::UIEvent;

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
        menu.choose(Some(window), None::<&gio::Cancellable>, |_| {

        });
        
        let model = Self {};
        
        model
    }
}
