extern crate gtk;
extern crate argparse;

use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition};

use argparse::{ArgumentParser, Store, StoreTrue};

const EXIT_NORMAL: i32 = 0;
const EXIT_ERROR: i32 = 1;

const PADDING: u32 = 10;

struct Settings {
    title: String,
    is_cancelable: bool,
}

impl Settings {
    
    fn new() -> Settings {
        Settings {
            title: String::from("Choose an option"),
            is_cancelable: false,
        }
    }

    fn from_args() -> Settings {
        
        let mut settings = Settings::new();

        {
            let mut parser = ArgumentParser::new();
            parser.set_description("Create a GTK based button dialog via command line");
    
            parser.refer(&mut settings.title)
                .add_option(&["-t", "--title"], Store,
                "Add a title to the window.");

            parser.refer(&mut settings.is_cancelable)
                .add_option(&["-c", "--cancelable"], StoreTrue,
                "Add cancel option to window. Defaults to exit code 0.");
    
            parser.parse_args_or_exit(); 
        }

        settings

    }

}

struct App<'a> {
    settings: &'a Settings,
    buttons: Vec<(i32, gtk::Button)>,
}

impl<'a> App<'a> {

    fn new(settings: &'a Settings, labels: Vec<&str>) -> App<'a> {
        let mut app = App {
            settings,
            buttons: Vec::new(),
        }; 

        for (i, label) in labels.iter().enumerate() {
            app.buttons.push((1 + EXIT_ERROR + i as i32, gtk::Button::new_with_label(label)));
        }
        
        if settings.is_cancelable {
            app.buttons.push((EXIT_NORMAL, gtk::Button::new_with_label("Cancel")));
        }

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
    
    window.set_title(&app.settings.title);

    let component = gtk::Box::new(gtk::Orientation::Vertical, 5);
   
    for &(fcode, ref button) in &app.buttons {
        button.connect_clicked(move |_| {
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
    let settings = Settings::from_args();

    if gtk::init().is_err() {
        println!("Failed to inizialite gtk");
        return;
    } 

    let mut app = App::new(&settings, labels);
    run(&mut app);

}
