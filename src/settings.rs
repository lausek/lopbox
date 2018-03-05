extern crate gtk;
extern crate argparse;
extern crate serde_json;

//use self::gtk::{Button};

use self::argparse::{ArgumentParser, Store, StoreTrue};

use self::serde_json::{Map, Value};

type SelectOption = (i32, gtk::Button);

pub struct Settings {
    pub title: String,
    pub is_cancelable: bool,
    pub buttons: Vec<SelectOption>,
}

impl Settings {

    fn new() -> Settings {
        Settings {
            title: String::from("Choose an option"),
            is_cancelable: false,
            buttons: Vec::new(),
        }
    }

    pub fn from_args() -> Result<Settings, &'static str> {

        let mut settings = Settings::new();

        let mut buttons_raw = String::new();

        {
            let mut parser = ArgumentParser::new();
            parser.set_description("Create a GTK based button dialog via command line");

            parser.refer(&mut settings.title)
                .add_option(&["-t", "--title"], Store,
                        "Add a title to the window.");

            parser.refer(&mut settings.is_cancelable)
                .add_option(&["-c", "--cancelable"], StoreTrue,
                        "Add cancel option to window. Defaults to exit code 0.");

            parser.refer(&mut buttons_raw)
                .add_option(&["-b", "--buttons"], Store,
                        r#"Specify the buttons as objects inside a JSON array. 
                        Each element can use the attributes:
                            code  - return code if this button is clicked; 
                            label - text to display"#)
                .required();

            parser.parse_args_or_exit(); 
        }

        match Settings::get_buttons(buttons_raw) {
            Ok(ls) => {
                settings.buttons = ls;

                Ok(settings)
            }, 
            Err(msg) => Err(msg), 
        } 

    }

    fn get_code(obj: &Map<String, Value>) -> i32 {
        
        if let Some(json_button) = obj.get("code") {
            match json_button {
                &Value::Number(ref n) => n.as_i64().unwrap() as i32,
                &Value::String(ref s) => s.parse::<i32>().unwrap(),
                _ => 0,
            }
        } else {
            0
        }
    
    }

    fn get_buttons(raw: String) -> Result<Vec<SelectOption>, &'static str> {
    
        match serde_json::from_str(raw.as_str()) {
            Ok(json_obj) => {
                
                if let Value::Array(json_arr) = json_obj {
                    
                    let mut buttons: Vec<SelectOption> = Vec::new();
                
                    for val in json_arr {
                        
                        if let Value::Object(json_button) = val {
                        
                            let code = Settings::get_code(&json_button);
                            let label = match json_button.get("label") {
                                Some(obj) => match obj { 
                                    &Value::String(ref s) => s.clone(),
                                    _ => format!("Option {}", code),
                                },
                                _ => format!("Option {}", code),
                            };

                            buttons.push((code, gtk::Button::new_with_label(label.as_str())));

                        } else {
                                
                            println!("'{}' is not interpretable as button", val);

                        }

                    }

                    Ok(buttons)
                
                } else {
                    
                    Err("Given json is not an array of objects")

                }

            },
            Err(err) => { println!("{:?}", err); Err("Error") }
        }
    
    }

}
