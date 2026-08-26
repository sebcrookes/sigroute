use gtk4::Button;
use libadwaita::Dialog;

pub trait OptionPicker {
    /// Returns whether or not the picker has been completed.
    fn is_now_completed(&self) -> bool;

    /// Returns the JSONified selections the user made in the picker.
    fn get_json(&self) -> String;

    /// Returns a clone of the submit button.
    fn get_submit_button(&self) -> Button;

    /// Returns a human readable summary of the selections the user
    /// made in the picker.
    fn get_summary_text(&self) -> String;

    /// Displays the picker on top of the provided parent widget.
    fn display(&self, parent: &Dialog);

    /// Closes the picker.
    fn close(&self);
}
