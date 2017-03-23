use result::Result;
use super::action_id::ActionId;

/// Action trait.
pub trait Action {
    /// Get the action ID.
    fn id(&self) -> ActionId;

    /// Get the name of the action.
    /// This is a short string that should make clear to the user what the
    /// action does.
    fn name(&self) -> &'static str;

    /// Invoke the action.
    ///
    /// A boolean is returned on success which defines whether the action has
    /// been consumed. `true` if the action is consumed, `false` if not.
    /// An error is returned if the action fails.
    fn invoke(&self) -> Result<bool>;
}
