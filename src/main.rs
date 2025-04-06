use gdk::prelude::*;
use gtk::prelude::*;
use gtk::CssProvider;
use gtk::StyleContext;
use relm4::prelude::*;

struct App {}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        main_window = gtk::ApplicationWindow {
            set_default_size: (380, 620),
            set_resizable: false,
            set_title: Some("Blitz"),

            gtk::Box {
                add_css_class: "box1",
                set_orientation: gtk::Orientation::Vertical,
                set_align: gtk::Align::Start,

                gtk::Image {
                    add_css_class: "image-logo",
                    set_valign: gtk::Align::Center,
                    set_margin_top: 20,
                    set_margin_bottom: 20,
                    set_margin_start: 20,
                    set_margin_end: 20,
                    set_icon_name: Some("src/blitz.svg"),
                },

                gtk::Button {
                    add_css_class: "button_clear",
                    set_label: "Click me!",
                    connect_clicked => {
                        println!("Button clicked!");
                    },
                },

                gtk::Button {
                    set_label: "Click me too!",
                    connect_clicked => {
                        println!("Button clicked too!");
                    },


                },
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {};

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        // Load CSS
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("style.css"));
        StyleContext::add_provider_for_display(
            &gtk::gdk::Display::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.simple");
    app.run::<App>(());
}
