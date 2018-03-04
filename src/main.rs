extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition};

const PADDING: u32 = 10;

struct App {
    buttons: Vec<(i32, gtk::Button)>,
}

impl App {

    fn new(labels: Vec<&str>) -> App {
        let mut app = App {
            buttons: Vec::new(),
        }; 

        for l in labels {
            app.buttons.push((-1, gtk::Button::new_with_label(l)));
        }
        
        app.buttons.push((0, gtk::Button::new_with_label("Cancel")));

        app
    }

}

fn close() -> gtk::Inhibit {
    gtk::main_quit();
    Inhibit(false)
}

fn run(app: &mut App) {

    let window = Window::new(WindowType::Popup);
    window.set_keep_above(true);
    window.set_position(WindowPosition::Center);
    window.connect_delete_event(|_, _| {
        close()
    });

    let component = gtk::Box::new(gtk::Orientation::Vertical, 5);
   
    for &(fcode, ref button) in &app.buttons {
        button.connect_clicked(move |obj| {
            println!("{:?}", obj);
            close();
            std::process::exit(fcode);
        });

        component.pack_start(button, true, true, PADDING);
    }

    component.show_all();

    window.add(&component);

    window.show_all();

    gtk::main();

}

fn main() {

    let labels = vec!["Lock", "Reboot", "Shutdown"];

    if gtk::init().is_err() {
        println!("Failed to inizialite gtk");
        return;
    }   

    let mut app = App::new(labels);
    run(&mut app);


}
