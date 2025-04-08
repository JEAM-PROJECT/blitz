use gtk::prelude::*;
use gtk::{glib, CssProvider};
use relm4::gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    gio::{Cancellable, MemoryInputStream},
    StyleContext,
};
use relm4::prelude::*;

use sysinfo::{
    Components, Disks, Networks, System,
};

#[derive(Default)]
pub struct Process {
    first_process: Option<String>,
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
        let mut sys = System::new_all();
        sys.refresh_all();

        let first_process = sys
            .processes()
            .values()
            .next()
            .map(|process| format!("{:?} \nCPU {:.2}% \nRAM {} GB", process.name(), process.cpu_usage(), process.memory()/1048576));

        let model = Process { first_process };
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
            add_css_class: "process",
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 10,

            append = &gtk::Label {
                set_label: "Process Administrator",
                set_margin_top: 10,
                set_margin_bottom: 10,
            },

            append = &gtk::Label {
                #[watch]
                set_label: model.first_process.as_deref().unwrap_or("No processes found"),
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
