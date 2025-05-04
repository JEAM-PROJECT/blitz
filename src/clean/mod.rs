// Import GTK libraries and traits
use gtk::prelude::*; // Core GTK traits
use gtk::CssProvider; // For applying CSS styling to widgets
use relm4::gtk::{
    gdk::Texture,                          // For handling image textures
    gdk_pixbuf::Pixbuf,                    // For image processing and loading
    gio::{Cancellable, MemoryInputStream}, // For stream operations
    glib,
    StyleContext, // For GLib types and styling context
};
use relm4::prelude::*; // Core Relm4 components and traits
use std::process::Command; // For executing shell commands
use sysinfo::{Disks, System}; // For system information access

/// This function retrieves the disk information.
///
/// Uses sysinfo crate to get information about available disks
/// and returns the total and available space for the first disk.
///
/// # Returns
/// * `Some(DiskInfo)` - Information about the disk if available
/// * `None` - If no disks are found
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

/// This function loads an image from the file system and creates a Texture from it.
///
/// # Arguments
/// * `picture` - Path to the image file to load
///
/// # Returns
/// * `Texture` - GTK Texture object that can be used in GTK widgets
///
/// # Panics
/// * If the image file cannot be read or processed
pub fn embedded_logo(picture: &str) -> Texture {
    let bytes = std::fs::read(picture).expect("Failed to read image file");
    let g_bytes = glib::Bytes::from(&bytes);
    let stream = MemoryInputStream::from_bytes(&g_bytes);
    let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE).unwrap();
    Texture::for_pixbuf(&pixbuf)
}

/// Struct to hold disk information.
///
/// Contains the total and available disk space in bytes.
#[derive(Default)]
struct DiskInfo {
    /// Total space available on the disk in bytes
    total_space: u64,
    /// Available free space on the disk in bytes
    available_space: u64,
}

/// Main struct for the Clean component.
///
/// This is the main component for the system cleaner interface
/// that displays disk information and cleanup options.
#[derive(Default)]
pub struct Clean {
    /// Information about the disk storage
    disk_info: DiskInfo,
}

/// Implementation of the Clean component for Relm4.
///
/// This component creates a GTK UI for system cleaning functionality.
#[relm4::component(pub)]
impl SimpleComponent for Clean {
    type Init = (); // No initialization parameters needed
    type Input = (); // No input messages handled
    type Output = (); // No output messages produced

    /// Defines the UI layout for the Clean component.
    ///
    /// Creates a vertical box layout with:
    /// - Title header
    /// - Status section
    /// - Storage usage information with progress bar
    /// - Quick action buttons for cleaning tasks
    view! {
        gtk::Box {
            add_css_class: "cleaner_box",
            set_orientation: gtk::Orientation::Vertical,

            // Main title
            append = &gtk::Label {
                add_css_class: "text_principal",
                set_label: "Blitz System Cleaner",
            },

            // Status section
            append = &gtk::Box {
                add_css_class: "status_box",
                set_orientation: gtk::Orientation::Vertical,

                append = &gtk::Label {
                    add_css_class: "status_text",
                    set_label: "Status",
                },
            },

            // Storage usage display section
            append = &gtk::Box {
                add_css_class: "storage_box",
                set_orientation: gtk::Orientation::Vertical,

                // Storage header with icon
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
                        set_paintable: Some(&embedded_logo("src/assets/64x64/storage.png")),
                    },
                },

                // Storage usage level bar - visualizes used vs total space
                append = &gtk::LevelBar {
                    set_align: gtk::Align::Start,
                    add_css_class: "storage_bar",
                    set_value: (model.disk_info.total_space - model.disk_info.available_space) as f64,
                    set_min_value: 0.0,
                    set_max_value: model.disk_info.total_space as f64,
                },

                // Storage usage text information
                append = &gtk::Label {
                    set_align: gtk::Align::Start,
                    add_css_class: "size_text",
                    set_label: &format!("{} GB used of {} GB", (model.disk_info.total_space - model.disk_info.available_space), model.disk_info.total_space)
                },
            },

            // Quick actions section
            append = &gtk::Box {
                add_css_class: "cleaner_actions_box",
                set_orientation: gtk::Orientation::Vertical,

                append = &gtk::Label {
                    set_align: gtk::Align::Start,
                    add_css_class: "cleaner_text",
                    set_label: "Quick Actions",
                },

                // Cache cleaning action button and labels
                append = &gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    add_css_class: "cleaner_cache_box",

                    // Button with trash icon to clear cache
                    append = &gtk::Button {
                        set_align: gtk::Align::Start,
                        add_css_class: "cache_button",
                        set_child: Some(&gtk::Image::from_icon_name("trash-symbolic")),
                        // Execute shell command to delete cache files when clicked
                        connect_clicked => move |_| {
                            let _output = Command::new("sh")
                                .arg("-c")
                                .arg("rm -rf ~/.cache/*")
                                .output()
                                .expect("Failed to execute command");
                        },
                    },

                    // Cache clear action label
                    append = &gtk::Label {
                        set_align: gtk::Align::Start,
                        add_css_class: "cleaner_text_cache",
                        set_label: "Clear cache",
                    },

                    // Description of cache clear action
                    append = &gtk::Label {
                        set_align: gtk::Align::Start,
                        add_css_class: "cleaner_text_cache_desc",
                        set_label: "Free up temporaly files",
                    },
                },
            },
        }
    }

    /// Initialize the Clean component.
    ///
    /// This method:
    /// 1. Retrieves disk information
    /// 2. Converts bytes to GB for display
    /// 3. Creates the UI widgets
    /// 4. Applies CSS styling
    ///
    /// # Returns
    /// * `ComponentParts` - The initialized model and widgets
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
