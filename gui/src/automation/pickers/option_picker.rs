use gtk4::Button;

pub trait OptionPicker {
    fn is_now_completed(&self) -> bool;
    fn get_json(&self) -> String;
    fn get_submit_button(&self) -> Button;
    fn get_summary_text(&self) -> String;
    fn close(&self);
}
