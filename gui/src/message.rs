pub enum Message {
    Initialisation,
    AddedAutomation,
    ChangedAutomation(i64), // The ID of the automation changed to
}
