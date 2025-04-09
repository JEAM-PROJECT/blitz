use gtk::prelude::*;
use gtk::{glib, CssProvider};
use relm4::gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    gio::{Cancellable, MemoryInputStream},
    StyleContext,
};
use relm4::prelude::*;
use glib::clone;

use sysinfo::{
    Components, Disks, Networks, System,
};

mod process_info; // Importar el módulo process_info
use process_info::get_processes_info; // Importar la función

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

        // Crear dinámicamente las filas de la lista
        let list_box = gtk::ListBox::new();
        
        for process in &model.processes {
            let row = gtk::ListBoxRow::new();
            let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    
            // Etiqueta para mostrar el nombre del proceso
            let label = gtk::Label::new(Some(process));
            label.set_margin_all(5);
    
            // Botón para cada proceso
            let button = gtk::Button::with_label("X");
            button.connect_clicked(clone!(@strong process => move |_| {
                println!("Button clicked for process: {}", process);
            }));
    
            // Agregar la etiqueta y el botón al contenedor horizontal
            hbox.append(&label);
            hbox.append(&button);
    
            // Establecer el contenedor como hijo de la fila
            row.set_child(Some(&hbox));
            list_box.append(&row);
        }

        root.append(&gtk::Label::new(Some("Process Administrator")));
        root.append(&list_box);
        
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
        }
    }
}
