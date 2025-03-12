use gtk::{gdk::Display, glib, Application, ApplicationWindow};
use gtk::{prelude::*, CssProvider};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);
    app.connect_startup(|_| load_css());

    // Run the application
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let ui_src = include_str!("window.ui");
    let builder = gtk::Builder::from_string(ui_src);

    let window: ApplicationWindow = builder
        .object::<gtk::ApplicationWindow>("window")
        .expect("Couldn't get window");
    app.add_window(&window);
    window.present();
}
