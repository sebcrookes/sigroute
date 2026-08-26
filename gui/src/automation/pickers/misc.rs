//! This module provides functionality which is used
//! across multiple pickers, to reduce code repetition.

use gtk4::{CheckButton, SpinButton, prelude::WidgetExt};
use libadwaita::{ActionRow, prelude::ActionRowExt};

/// Creates an ActionRow for dialogs, composed of
/// a title, a subtitle and a SpinButton with the
/// provided minimum and maximum values.
pub fn create_spinbtn_row(title: &str, subtitle: Option<&str>, min: i64, max: i64) -> (ActionRow, SpinButton) {
    let row = ActionRow::builder()
        .title(title)
        .build();

    if subtitle.is_some() {
        row.set_subtitle(subtitle.unwrap());
    }

    let picker = create_spinbtn(min, max);
    row.add_suffix(&picker);

    return (row, picker);
}

/// Creates an ActionRow for dialogs, composed of a
/// title, a subtitle and a CheckButton.
pub fn create_chkbtn_row(title: &str, subtitle: Option<&str>) -> (ActionRow, CheckButton) {
    let button = CheckButton::new();
    let row = ActionRow::builder()
        .title(title)
        .build();

    if subtitle.is_some() {
        row.set_subtitle(subtitle.unwrap());
    }
    
    row.add_suffix(&button);

    return (row, button);
}

/// Constructs a SpinButton used in the various
/// pickers, with the provided minimum and maximum
/// values. The entered values must be numeric and
/// integers.
pub fn create_spinbtn(min: i64, max: i64) -> SpinButton {
    let spin_button = SpinButton::with_range(min as f64, max as f64, 1.0);
    spin_button.set_numeric(true);
    spin_button.set_digits(0);
    spin_button.set_snap_to_ticks(true);
    spin_button.set_margin_top(8);
    spin_button.set_margin_bottom(8);

    return spin_button;
}
