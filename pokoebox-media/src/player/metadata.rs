/// Source metadata.
///
/// Metadata of the current playing item in a source, such as the current track.
#[derive(Debug, Clone, Default)]
pub struct Metadata {
    /// Albun name.
    album_name: Option<String>,

    /// Artist names.
    artists: Vec<String>,

    /// Track title name.
    title: Option<String>,
}
