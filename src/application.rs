use crate::config;
use crate::window::MainApplicationWindow;
use gio::ApplicationFlags;
use glib::clone;
use glib::WeakRef;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib};
use gtk_macros::action;
use log::{debug, info};
use once_cell::sync::OnceCell;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PipeCtrlApplication {
        pub window: OnceCell<WeakRef<MainApplicationWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PipeCtrlApplication {
        const NAME: &'static str = "ExampleApplication";
        type Type = super::PipeCtrlApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for PipeCtrlApplication {}

    impl gio::subclass::prelude::ApplicationImpl for PipeCtrlApplication {
        fn activate(&self, app: &Self::Type) {
            debug!("GtkApplication<ExampleApplication>::activate");

            let priv_ = PipeCtrlApplication::from_instance(app);
            if let Some(window) = priv_.window.get() {
                let window = window.upgrade().unwrap();
                window.show();
                window.present();
                return;
            }

            app.set_resource_base_path(Some("/com/github/personalizedrefrigerator/pipes/"));
            app.setup_css();

            let window = MainApplicationWindow::new(app);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            app.setup_gactions();
            app.setup_accels();

            app.get_main_window().present();
        }

        fn startup(&self, app: &Self::Type) {
            debug!("GtkApplication<PipeCtrlApplication>::startup");
            self.parent_startup(app);
        }
    }

    impl GtkApplicationImpl for PipeCtrlApplication {}
}

glib::wrapper! {
    pub struct PipeCtrlApplication(ObjectSubclass<imp::PipeCtrlApplication>)
        @extends gio::Application, gtk::Application, @implements gio::ActionMap, gio::ActionGroup;
}

impl PipeCtrlApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &Some(config::APP_ID)),
            ("flags", &ApplicationFlags::empty()),
        ])
        .expect("Application initialization failed...")
    }

    fn get_main_window(&self) -> MainApplicationWindow {
        let priv_ = imp::PipeCtrlApplication::from_instance(self);
        priv_.window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        action!(
            self,
            "quit",
            clone!(@weak self as app => move |_, _| {
                // This is needed to trigger the delete event
                // and saving the window state
                app.get_main_window().close();
                app.quit();
            })
        );

        // About
        action!(
            self,
            "about",
            clone!(@weak self as app => move |_, _| {
                app.show_about_dialog();
            })
        );
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<primary>q"]);
        self.set_accels_for_action("win.show-help-overlay", &["<primary>question"]);
    }

    fn setup_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/com/github/personalizedrefrigerator/pipes/style.css");
        if let Some(display) = gdk::Display::default() {
            gtk::StyleContext::add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn show_about_dialog(&self) {
        let dialog = gtk::AboutDialogBuilder::new()
            .program_name("Pipes")
            .logo_icon_name(config::APP_ID)
            // Insert your license of choice here
            .license_type(gtk::License::Gpl30)
            .website("https://github.com/personalizedrefrigerator/pipes")
            .version(config::VERSION)
            .transient_for(&self.get_main_window())
            .modal(true)
            .authors(vec!["Henry Heino".into()])
            .artists(vec!["Henry Heino".into()])
            .build();

        dialog.show();
    }

    pub fn run(&self) {
        info!("Pipes ({})", config::APP_ID);
        info!("Version: {} ({})", config::VERSION, config::PROFILE);
        info!("Datadir: {}", config::PKGDATADIR);

        ApplicationExtManual::run(self);
    }
}
