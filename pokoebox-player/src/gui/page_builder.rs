use super::gtk;
use super::gtk::*;

/// Page builder trait.
/// This trait can be used to create new page builders.
/// A page builder is used to build a new page.
pub trait PageBuilder {

    /// Get the name of the page.
    fn page_name(&self) -> &'static str;

    /// Build the actual page gui on the GTK widget of the given page.
    fn build_page(&self);

    /// Get the GTK widget that represents the page.
    fn gtk_widget(&self) -> &gtk::Grid;
}

/// Page builder helper.
pub struct PageBuilderHelper { }

impl PageBuilderHelper {

    /// Create a GTK container to build the page upon.
    /// The page is automatically build using `build_page` as soon as the container is created.
    /// The GTK container widget is returned.
    pub fn create_page_container() -> gtk::Grid {
        // Create the page container
        let container = gtk::Grid::new();

        // Configure the container
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_border_width(8);

        container
    }
}
