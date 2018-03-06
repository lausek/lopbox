extern crate gtk;

use gtk::prelude::*;
use gtk::{StyleContext, Window, WindowType, WindowPosition};

mod settings;
use settings::Settings;

const EXIT_NORMAL: i32 = 0;
const EXIT_ERROR: i32 = 1;
const PADDING: u32 = 10;

struct App<'a> {
    settings: &'a mut Settings,
}

impl<'a> App<'a> {

    fn new(settings: &'a mut Settings) -> App<'a> {
        let app = App {
            settings,
        }; 
        
        if app.settings.is_cancelable {
            app.settings.buttons.push((EXIT_NORMAL, gtk::Button::new_with_label("Cancel")));
        }

        app
    }

}

fn close() -> gtk::Inhibit {
    gtk::main_quit();
    Inhibit(false)
}

fn run(app: &App) {
    
    let provider = gtk::CssProvider::new();
    
    {
        let css_string = format!("box, button {{border-radius:0;background-image:none;color:{};background-color:{}; }} box {{padding:15px;}}",
                                app.settings.foreground,
                                app.settings.background);
        provider.load_from_data(css_string.as_bytes()).ok();
    }

    let window = Window::new(WindowType::Popup);
    window.set_keep_above(true);
    window.set_position(WindowPosition::Center);
    window.connect_delete_event(|_, _| {
        close()
    });
    window.set_title(&app.settings.title);

    if let Some(context) = window.get_style_context() {
        let screen = context.get_screen().unwrap();
        StyleContext::add_provider_for_screen(&screen, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION); 
    }
    
    let component = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        
    if let &Some(ref info) = &app.settings.info {
        component.pack_start(&gtk::Label::new(info.as_str()), true, true, PADDING);
    }
   
    for &(fcode, ref button) in &app.settings.buttons {
        button.connect_clicked(move |_| {
            close();
            std::process::exit(fcode);
        });
    
        component.pack_start(button, true, true, PADDING);
    }

    component.set_margin_top(2);
    component.set_margin_bottom(2);
    component.set_margin_left(2);
    component.set_margin_right(2);
    component.show_all();

    window.add(&component);
    
    window.show_all();

    gtk::main();

}

fn main() {
    
    gtk::init().unwrap_or_else(|_| panic!("Failed to inizialite gtk"));    

    match Settings::from_args() {
        Ok(mut settings) => {
            let app = App::new(&mut settings);
            run(&app);
        },
        Err(msg) => println!("{}", msg), 
    }
    
}
