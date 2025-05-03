use gtk::prelude::*;
use gtk::CssProvider;
use relm4::gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    gio::{Cancellable, MemoryInputStream},
    glib, StyleContext,
};
use relm4::prelude::*;

mod process_info; // Importar el módulo process_info
use process_info::get_processes_info; // Importar la función

pub fn embedded_logo(picture: &str) -> Texture {
    let bytes = std::fs::read(picture).expect("Failed to read image file");
    let g_bytes = glib::Bytes::from(&bytes);
    let stream = MemoryInputStream::from_bytes(&g_bytes);
    let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE).unwrap();
    Texture::for_pixbuf(&pixbuf)
}

#[derive(Default)]
pub struct Process {
    processes: Vec<String>, // Cambiar a una lista de procesos
}

#[relm4::component(pub)]
impl SimpleComponent for Process {
    type Init = ();
    type Input = ();
    type Output = ();

    fn init(
        _params: Self::Init,
        root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let processes = get_processes_info(); // Usar la función para obtener los procesos
        let model = Process { processes };

        let widgets = view_output!();

        let provider = CssProvider::new();
        provider.load_from_data(include_str!("style.css"));
        StyleContext::add_provider_for_display(
            &gtk::gdk::Display::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        relm4::ComponentParts { model, widgets }
    }

    view! {
        gtk::Box {
            add_css_class: "cleaner_box",
            set_orientation: gtk::Orientation::Vertical,

            append = &gtk::Label {
                add_css_class: "principal_text",
                set_label: "Blitz System Cleaner",
            },

            append = &gtk::Box {
                add_css_class: "principal_box",
                set_orientation: gtk::Orientation::Vertical,

                append = &gtk::Box {
                    add_css_class: "process_box",
                    set_orientation: gtk::Orientation::Horizontal,

                    append = &gtk::Picture {
                        set_paintable: Some(&embedded_logo("src/assets/64x64/process.png")),
                    },

                    append = &gtk::Label {
                        add_css_class: "process_text",
                        set_label: "Process Administrator",
                    },
                },

                append = &gtk::Button {
                    add_css_class: "process_button",
                    set_label: "Kill Process(es) (0)",

                    connect_clicked => move |_| {
                        println!("Kill process list clicked");
                    },
                },

                append = &gtk::Box {
                    add_css_class: "process_info_box",
                    set_orientation: gtk::Orientation::Vertical,

                    append = &gtk::Box {
                        add_css_class: "process_header_box",
                        set_orientation: gtk::Orientation::Horizontal,

                        append = &gtk::Picture {
                            add_css_class: "process_info_icon",
                            set_paintable: Some(&embedded_logo("src/assets/64x64/storage.png")),
                        },

                        append = &gtk::Label {
                            add_css_class: "process_info_name_text",
                            set_label: "Chrome",
                        },

                        append = &gtk::Box {
                            set_hexpand: true,
                            set_halign: gtk::Align::Fill,
                        },

                        append = &gtk::Button {
                            add_css_class: "kill_process_button",
                            set_child: Some(&gtk::Image::from_icon_name("window-close")),

                            connect_clicked => move |_| {
                                println!("Kill Process button clicked");
                            },
                        }
                    },

                    append = &gtk::Box {
                        add_css_class: "process_info_tag_box",
                        set_orientation: gtk::Orientation::Horizontal,

                        append = &gtk::Label {
                            add_css_class: "process_info_tag_text",
                            set_label: "CPU:",
                        }
                    },

                    append = &gtk::Box {
                        add_css_class: "process_info_tag_box",
                        set_orientation: gtk::Orientation::Horizontal,

                        append = &gtk::Label {
                            add_css_class: "process_info_tag_text",
                            set_label: "Memory:",
                        },

                        append = &gtk::Box {
                            set_hexpand: true,
                            set_halign: gtk::Align::Fill,
                        },

                        append = &gtk::Label {
                            add_css_class: "process_info_value_text",
                            set_label: "1.2 GB",
                        }
                    },

                    append = &gtk::Box {
                        add_css_class: "process_info_tag_box",
                        set_orientation: gtk::Orientation::Horizontal,

                        append = &gtk::Label {
                            add_css_class: "process_info_tag_text",
                            set_label: "Type:",
                        },

                        append = &gtk::Box {
                            set_hexpand: true,
                            set_halign: gtk::Align::Fill,
                        },

                        append = &gtk::Label {
                            add_css_class: "process_info_type_text",
                            set_label: "Browser",
                        }
                    }
                }
            }
        }
    }
}
