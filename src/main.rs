extern crate argparse;
extern crate gtk;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod app;
mod settings;

use app::*;
use settings::*;

use gtk::prelude::*;
use gtk::{StyleContext, Window, WindowPosition, WindowType};

const PADDING: u32 = 10;

fn close() -> gtk::Inhibit {
    gtk::main_quit();
    Inhibit(false)
}

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to inizialite gtk"));

    match Settings::from_args() {
        Ok(mut settings) => {
            settings.add_stdin();
            App::new(settings).run();
        }
        Err(msg) => println!("{}", msg),
    }
}
