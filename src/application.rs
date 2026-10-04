use libadwaita as adw;
use adw::prelude::*;

const APP_ID: &str = "com.github.mehmet.typwriter";

pub fn run() {
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();
    
    app.connect_startup(|_| {
        load_css();
    });
    
    app.connect_activate(|app| {
        let window = crate::window::build_window(app);
        window.present();
    });
    
    app.run();
}

fn load_css() {
    let provider = gtk4::CssProvider::new();
    let css_data = include_str!("../data/style.css");
    provider.load_from_string(css_data);
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not get default display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
