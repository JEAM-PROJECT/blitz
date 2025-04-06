#![allow(deprecated)]

use std::convert::identity;

use gtk::prelude::*;
use relm4::prelude::*;

struct AppModel {
    counter: u8,
}

struct Header;

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = ();
    type Output = ();
    type Root = gtk::Window;

    view! {
        gtk::HeaderBar {
            #[wrap(Some)]
            set_title_widget = &gtk::Box {
                add_css_class: relm4::css::LINKED,
                append: group = &gtk::ToggleButton {
                    set_label: "View",
                    set_active: true,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(()).unwrap();
                        }
                    },
                },
                append = &gtk::ToggleButton {
                    set_label: "Edición",
                    set_group: Some(&group),
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(()).unwrap();
                        }
                    },
                },
                append = &gtk::ToggleButton {
                    set_label: "Export",
                    set_group: Some(&group),
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(()).unwrap();
                        }
                    },
                },
            }
        }
    }

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("HeaderBar Example")
            .default_width(300)
            .default_height(200)
            .build();
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel { counter: 0 };
        let header = Header;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.widget_template");
    app.run::<AppModel>(0);
}
