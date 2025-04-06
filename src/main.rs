use gtk::prelude::*;
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
            set_title: Some("Blitz"),
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

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.simple");
    app.run::<App>(());
}
