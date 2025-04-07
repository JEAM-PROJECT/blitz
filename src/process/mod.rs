use gtk::prelude::*;
use gtk::{glib, CssProvider};
use relm4::gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    gio::{Cancellable, MemoryInputStream},
    StyleContext,
};
use relm4::prelude::*;

#[derive(Default)]
pub struct Process;

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
        let model = Process {};
        let widgets = view_output!();

        relm4::ComponentParts { model, widgets }
    }

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 10,

            append = &gtk::Label {
                set_label: "Hello, Process",
                set_margin_top: 10,
                set_margin_bottom: 10,
            },

            append = &gtk::Button {
                set_label: "Proceso 1",
                connect_clicked => |_| {
                    println!("Button clicked!");
                },
            },
        }
    }
}
