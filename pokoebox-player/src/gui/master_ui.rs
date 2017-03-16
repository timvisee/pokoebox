use super::header::Header;
use super::page_container::PageContainer;

/// The actual UI inside the master container, with a header and page container to show the application's UI in.
pub struct MasterUi {
    header: Header,
    page_container: PageContainer
}

impl MasterUi {

    /// Construct a new master UI.
    pub fn new() -> Self {
        MasterUi {
            header: Self::build_header(),
            page_container: Self::build_page_container()
        }
    }

    /// Build the header.
    fn build_header() -> Header {
        Header::new()
    }

    /// Build the page container.
    fn build_page_container() -> PageContainer {
        PageContainer::new()
    }

    /// Get the header.
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Get the GTK page container widget.
    pub fn page_container(&self) -> &PageContainer {
        &self.page_container
    }

    /// Get the GTK page container widget.
    pub fn mut_page_container(&mut self) -> &mut PageContainer {
        &mut self.page_container
    }
}
