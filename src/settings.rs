use argparse::*;

type SelectOption = (i32, gtk::Button);

const EXIT_NORMAL: i32 = 0;

#[derive(Serialize, Deserialize)]
struct JsonButton {
    label: String,
    code: i32,
}

pub struct Settings {
    pub title: String,
    pub info: Option<String>,
    pub cancelable: bool,
    pub buttons: Vec<SelectOption>,
    pub foreground: String,
    pub background: String,
}

impl Settings {
    fn new() -> Settings {
        Settings {
            title: String::from("Choose an option"),
            info: None,
            cancelable: false,
            buttons: Vec::new(),
            foreground: String::from("#FFFFFF"),
            background: String::from("#000000"),
        }
    }

    pub fn from_args() -> Result<Settings, String> {
        let mut settings = Settings::new();
        let mut buttons_raw = String::new();

        {
            let mut parser = ArgumentParser::new();
            parser.set_description("Create a GTK based button dialog via command line");

            parser.refer(&mut settings.title).add_option(
                &["-t", "--title"],
                Store,
                "Add a title to the window.",
            );

            parser.refer(&mut settings.info).add_option(
                &["-i", "--info"],
                StoreOption,
                "Add an info message to the window.",
            );

            parser.refer(&mut settings.cancelable).add_option(
                &["-c", "--cancelable"],
                StoreTrue,
                "Add cancel option to window. Defaults to exit code 0.",
            );

            parser
                .refer(&mut buttons_raw)
                .add_option(
                    &["-o", "--options"],
                    Store,
                    r#"Specify the buttons as objects inside a JSON array. 
                        Each element can use the attributes:
                            code  - return code if this button is clicked; 
                            label - text to display"#,
                )
                .required();

            parser.refer(&mut settings.foreground).add_option(
                &["-f", "--foreground"],
                Store,
                "Set the foreground color of the dialog. (hexadecimal)",
            );

            parser.refer(&mut settings.background).add_option(
                &["-b", "--background"],
                Store,
                "Set the background color of the dialog. (hexadecimal)",
            );

            parser.parse_args_or_exit();
        }

        settings.buttons = Settings::get_buttons(buttons_raw)?;
        if settings.cancelable {
            settings
                .buttons
                .push((EXIT_NORMAL, gtk::Button::new_with_label("Cancel")));
        }

        Ok(settings)
    }

    fn get_buttons(raw: String) -> Result<Vec<SelectOption>, String> {
        match serde_json::from_str::<Vec<JsonButton>>(raw.as_str()) {
            Ok(buttons) => {
                let buttons = buttons
                    .iter()
                    .map(|jb| (jb.code, gtk::Button::new_with_label(jb.label.as_str())))
                    .collect::<Vec<SelectOption>>();
                Ok(buttons)
            }
            Err(err) => Err(format!("{}", err)),
        }
    }
}
