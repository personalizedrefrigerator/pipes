use graphene::Rect;
use gtk;
use gtk::gdk::RGBA;
use gtk::glib;

use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct PipeConnector;

#[glib::object_subclass]
impl ObjectSubclass for PipeConnector {
    const NAME: &'static str = "PipectlPipeConnector";
    type Type = super::PipeConnector;
    type ParentType = gtk::Button;
}

impl ObjectImpl for PipeConnector {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(&obj);
        obj.queue_draw();
    }
}

impl WidgetImpl for PipeConnector {
    fn snapshot(&self, _widget: &Self::Type, snapshot: &gtk::Snapshot) {
        let r = Rect::new(0.0, 0.0, 10.0, 20.0);
        let c = RGBA::builder()
            .red(0.0)
            .green(0.0)
            .blue(1.0)
            .alpha(1.0)
            .build();
        snapshot.append_color(&c, &r);
    }
}

impl ButtonImpl for PipeConnector {}
