mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct PipeConnector(ObjectSubclass<imp::PipeConnector>)
        @extends gtk::Button, gtk::Widget;
}

impl PipeConnector {
    pub fn new() -> Self {
        Object::new(&[("label", &"_This is a test")]).expect("Failed to create PipeConnector")
    }
}

impl Default for PipeConnector {
    fn default() -> Self {
        Self::new()
    }
}
