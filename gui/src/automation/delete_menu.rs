//! This module contains code to provide menus to assist in the
//! deletion of various structures, such as automations, triggers,
//! actions (and possibly more in the future). These are specified
//! by DeleteType, and these types are provided to DeleteMenu to
//! construct a UI dialog to ask for deletion confirmation.

use async_channel::Sender;
use gtk4::glib;
use libadwaita::{AlertDialog, ApplicationWindow, prelude::{AdwDialogExt, AlertDialogExt, AlertDialogExtManual}};

use crate::message::UIEvent::{self, DeletedAction, DeletedAutomation, DeletedTrigger};

/// Types of structures which can be deleted by DeleteMenu.
#[derive(PartialEq, Copy, Clone)]
pub enum DeleteType {
    /// DeleteMenu is deleting an automation, and expects "None" in the ID
    /// field of the constructor (as DeletedAutomation events always refer
    /// to the currently selected automation).
    Automation,

    /// DeleteMenu is deleting a trigger, and thus expects a trigger ID.
    Trigger,

    /// DeleteMenu is deleting an action, and thus expects an action ID.
    Action
}

impl DeleteType {
    /// Returns the name of the structure being deleted as a string.
    fn to_string(&self) -> &str {
        match self {
            DeleteType::Automation => "Automation",
            DeleteType::Trigger => "Trigger",
            DeleteType::Action => "Action"
        }
    }

    /// Returns whether or not this type requires an ID
    fn requires_id(&self) -> bool {
        [DeleteType::Trigger, DeleteType::Action].contains(self)
    }

    /// Returns a UI event which should be sent to signal the deletion
    /// of this structure, given an optional ID.
    /// 
    /// # Panics
    /// 
    /// This function will panic when an ID is required but not
    /// provided. See requires_id for whether or not this deletion type
    /// requires an ID.
    fn get_deletion_event(&self, id: Option<i64>) -> UIEvent {
        // Ensuring that if an ID is required, it has been provided
        if self.requires_id() && id.is_none() {
            panic!("Using DeleteMenu on the type {} requires an ID", self.to_string());
        }

        match self {
            DeleteType::Automation => DeletedAutomation(),
            DeleteType::Trigger => DeletedTrigger(id.unwrap()),
            DeleteType::Action => DeletedAction(id.unwrap())
        }
    }
}

/// UI element which uses a libadwaita AlertDialog to present the user
/// with a deletion confirmation, before carrying out that deletion.
/// 
/// It allows for three different types of structures to be deleted:
/// * deleting automations,
/// * deleting triggers, and
/// * deleting actions
#[derive(Clone)]
pub struct DeleteMenu {
    dialog: AlertDialog,
}

impl DeleteMenu {
    /// Creates a new DeleteMenu, given the UI event sender, the deletion
    /// type, and an ID, if required for deletion of that structure.
    /// 
    /// # Panics
    /// 
    /// This constructor will panic if an ID is required but is not given.
    /// Check DeleteType::requires_id for which types require IDs.
    pub fn new(sender: &Sender<UIEvent>, delete_type: DeleteType, id: Option<i64>) -> Self {
        let heading = format!("Delete {}", delete_type.to_string());

        // If an ID is required to delete this structure and one wasn't provided, panic
        if delete_type.requires_id() && id.is_none() {
            panic!("DeleteMenu expects an id for the provided DeleteType");
        }

        let body = format!("Are you sure you want to delete this {}?", delete_type.to_string().to_lowercase());

        // Constructing an alert dialog with "Cancel" and "Delete" buttons
        let menu = AlertDialog::builder()
            .heading(heading)
            .body(body)
            .build();

        menu.add_responses(&[
            ("cancel", "Cancel"),
            ("delete", "Delete")
        ]);
        menu.set_response_appearance("delete", libadwaita::ResponseAppearance::Destructive);

        /* Connecting a callback to actually delete the structure */
        let sender_clone = sender.clone();
        menu.connect_response(None, move |_, option| {
            if option == "delete" {
                let sender_clone = sender_clone.clone();
                glib::spawn_future_local(async move {
                    sender_clone.send(delete_type.get_deletion_event(id)).await.unwrap();
                });
            }
        });
        
        let model = Self {
            dialog: menu
        };
        
        model
    }

    /// Displays the DeleteMenu, given the window to display it on.
    pub fn display(&self, window: &ApplicationWindow) {
        self.dialog.present(Some(window));
    }
}
