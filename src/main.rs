use gtk::{gdk::Display, glib, Application, ApplicationWindow};
use gtk::{prelude::*, Button, CssProvider, Text};

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
    let button = Button::builder()
        .label("Click me!")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    button.add_css_class("button1");

    button.connect_clicked(|button| {
        button.set_label("Hello from my rust Apllication xd");
    });

    let text = Text::builder().editable(false).build();
    text.set_text("Hello World");
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .resizable(false)
        .default_width(420)
        .default_height(620)
        .title("RustClean")
        .child(&button)
        .build();

    // Present window
    window.present();
}
