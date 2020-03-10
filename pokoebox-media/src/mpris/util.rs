/// Finds difference between old and new iterator.
///
/// Returns two lists with `(added, removed)` items.
pub(crate) fn iter_diff<T>(old: Vec<T>, new: &[T]) -> (Vec<T>, Vec<T>)
where
    T: PartialEq + Eq + Clone,
{
    // Find diffs
    (
        new.iter().filter(|i| !old.contains(i)).cloned().collect(),
        old.into_iter().filter(|i| !new.contains(i)).collect(),
    )
}
