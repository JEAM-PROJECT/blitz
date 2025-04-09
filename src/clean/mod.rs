use gtk::prelude::*;
use gtk::CssProvider;
use relm4::gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    gio::{Cancellable, MemoryInputStream},
    glib, StyleContext,
};
use relm4::prelude::*;
use std::process::Command;
use sysinfo::{Disks, System};

//this function gets the disk information
fn get_disks() -> Option<DiskInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let disks = Disks::new_with_refreshed_list();
    if let Some(disk) = disks.list().first() {
        return Some(DiskInfo {
            total_space: disk.total_space(),
            available_space: disk.available_space(),
        });
    }
    None
}

//this function loads the image from the file system and creates a Texture from it
pub fn embedded_logo(picture: &str) -> Texture {
    let bytes = std::fs::read(picture).expect("Failed to read image file");
    let g_bytes = glib::Bytes::from(&bytes);
    let stream = MemoryInputStream::from_bytes(&g_bytes);
    let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE).unwrap();
    Texture::for_pixbuf(&pixbuf)
}

#[derive(Default)]
struct DiskInfo {
    total_space: u64,
    available_space: u64,
}

#[derive(Default)]
pub struct Clean {
    disk_info: DiskInfo,
}

#[relm4::component(pub)]
impl SimpleComponent for Clean {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::Box {
            add_css_class: "cleaner_box",
            set_orientation: gtk::Orientation::Vertical,

            append = &gtk::Label {
                add_css_class: "text_principal",
                set_label: "Blitz System Cleaner",
            },

            append = &gtk::Box {
                add_css_class: "status_box",
                set_orientation: gtk::Orientation::Vertical,

                append = &gtk::Label {
                    add_css_class: "status_text",
                    set_label: "Status",
                },
            },

            append = &gtk::Box {
                add_css_class: "storage_box",
                set_orientation: gtk::Orientation::Vertical,


                append = &gtk::Box {
                    set_align: gtk::Align::Start,
                    add_css_class: "storage_text_icon_box",
                    set_orientation: gtk::Orientation::Horizontal,

                    append = &gtk::Label {
                        add_css_class: "storage_text",
                        set_label: "Storage Usage",
                    },

                    append = &gtk::Picture {
                        add_css_class: "storage_icon",
                        set_paintable: Some(&embedded_logo("./src/assets/64x64/storage.png")),
                    },
                },


                append = &gtk::LevelBar {
                    set_align: gtk::Align::Start,
                    add_css_class: "storage_bar",
                    set_value: (model.disk_info.total_space - model.disk_info.available_space) as f64,
                    set_min_value: 0.0,
                    set_max_value: model.disk_info.total_space as f64,
                },

                append = &gtk::Label {
                    #[watch]
                    set_align: gtk::Align::Start,
                    add_css_class: "size_text",
                    set_label: &format!("{} GB used of {} GB", (model.disk_info.total_space - model.disk_info.available_space), model.disk_info.total_space)
                },
            },

            append = &gtk::Box {
                add_css_class: "cleaner_actions_box",
                set_orientation: gtk::Orientation::Vertical,

                append = &gtk::Label {
                    set_align: gtk::Align::Start,
                    add_css_class: "cleaner_text",
                    set_label: "Quick Actions",
                },

                append = &gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    add_css_class: "cleaner_cache_box",

                    append = &gtk::Button {
                        set_align: gtk::Align::Start,
                        add_css_class: "cache_button",
                        set_child: Some(&gtk::Image::from_icon_name("trash-symbolic")),
                        connect_clicked => move |_| {
                            let _output = Command::new("sh")
                                .arg("-c")
                                .arg("rm -rf ~/.cache/*")
                                .output()
                                .expect("Failed to execute command");
                        },
                    },

                    append = &gtk::Label {
                        set_align: gtk::Align::Start,
                        add_css_class: "cleaner_text_cache",
                        set_label: "Clear cache",
                    },

                    append = &gtk::Label {
                        set_align: gtk::Align::Start,
                        add_css_class: "cleaner_text_cache_desc",
                        set_label: "Free up temporaly files",
                    },


                }

            //append = &gtk::Button {
              //      add_css_class: "cleaner_button",
                //    set_label: "Clean Now",
                  //  connect_clicked => move |_| {
                    //    let _output = Command::new("sh")
                      //      .arg("-c")
                        //    .arg("rm -rf /tmp/*")
                          //  .output()
                            //.expect("Failed to execute command");
                            //},
                            //},
            },
        }
    }

    fn init(
        _params: Self::Init,
        root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let size = get_disks().unwrap_or_default();
        let model = Clean {
            disk_info: DiskInfo {
                total_space: size.total_space / 1024 / 1024 / 1024,
                available_space: size.available_space / 1024 / 1024 / 1024,
            },
        };
        let widgets = view_output!();

        // Load CSS
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("style.css"));
        StyleContext::add_provider_for_display(
            &gtk::gdk::Display::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        relm4::ComponentParts { model, widgets }
    }
}
