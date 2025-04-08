use gtk::prelude::*;
use gtk::{glib, CssProvider};
use relm4::gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    gio::{Cancellable, MemoryInputStream},
    StyleContext,
};
use relm4::prelude::*;

mod clean;
use clean::Clean;

mod process;
use process::Process;

struct App {
    mode: AppMode,
    clean_component: Controller<Clean>, // 👈 agrega esto
    process_component: Controller<Process>,
}

#[derive(Debug)]
enum AppMode {
    View1,
    View2,
}

#[derive(Debug)]
enum AppMsg {
    SetMode(AppMode),
}

fn embedded_logo() -> Texture {
    let bytes = include_bytes!("./assets/blitz.png");
    let g_bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = MemoryInputStream::from_bytes(&g_bytes);
    let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE).unwrap();
    Texture::for_pixbuf(&pixbuf)
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        main_window = gtk::ApplicationWindow {
            set_default_size: (380, 620),
            set_resizable: false,
            set_title: Some("Blitz"),

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,

                append = &gtk::Box {
                    add_css_class: "box1",
                    set_orientation: gtk::Orientation::Vertical,
                    set_align: gtk::Align::Start,
                    gtk::Image {
                        add_css_class: "logo",
                        set_paintable: Some(&embedded_logo()),
                    },

                    append: group = &gtk::Button {
                        add_css_class: "button_clear",
                        set_label: "Click me!",
                        connect_clicked[sender] => move |_| {
                                sender.input(AppMsg::SetMode(AppMode::View1));
                                println!("Button clicked!");
                        },
                    },

                    append = &gtk::Button {
                        add_css_class: "button_clear",
                        set_label: "Click me too!",
                        connect_clicked[sender] => move |_| {
                                sender.input(AppMsg::SetMode(AppMode::View2));
                                println!("Button clicked!");
                        }
                    },
                },

                append = &gtk::Box {
                    #[watch]
                    set_visible: matches!(model.mode, AppMode::View1),
                    set_orientation: gtk::Orientation::Vertical,

                    append = &gtk::Box {
                        append = &gtk::Box {
                            // Wrap Clean in a gtk::Box to satisfy the IsA<Widget> trait
                            append = &gtk::Box {
                                append = model.clean_component.widget(),

                            },
                        },
                    },
                },

                append = &gtk::Box {
                    #[watch]
                    set_visible: matches!(model.mode, AppMode::View2),
                    set_orientation: gtk::Orientation::Vertical,
                    set_align: gtk::Align::Center,
                    
                    append = &gtk::Box {
                        add_css_class: "view",
                        append = model.process_component.widget(),
                    }
                },
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let clean_component = Clean::builder().launch(()).detach();
        let process_component = Process::builder().launch(()).detach();
        let model = App {
            mode: AppMode::View1,
            clean_component,
            process_component,
        };

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

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::SetMode(mode) => {
                self.mode = mode;
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.simple");
    app.run::<App>(());
}
