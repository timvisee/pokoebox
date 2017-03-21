use super::action_id::ActionId;

/// Action trait.
pub trait Action {
    /// Get the action ID.
    fn id(&self) -> ActionId;

    /// Get the name of the action.
    /// This is a short string that should make clear to the user what the action does.
    fn name(&self) -> &'static str;
}