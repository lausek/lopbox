use super::*;

pub struct App {
    settings: Settings,
}

impl App {
    pub fn new(settings: Settings) -> App {
        App { settings }
    }

    pub fn run(&self) {
        let provider = gtk::CssProvider::new();

        {
            let f = &self.settings.foreground;
            let b = &self.settings.background;
            let css_string = format!(
                "box, button {{
                                    border-radius:0;
                                    background-image:none;
                                    color:{0};
                                    border-color:{0};
                                    background-color:{1};
                                 }} 
                                 label {{text-shadow:none}}
                                 window {{
                                    background-color:{0};
                                 }}
                                 box {{
                                    padding:15px;
                                 }}",
                f, b
            );
            provider.load_from_data(css_string.as_bytes()).ok();
        }

        let window = Window::new(WindowType::Popup);
        window.set_keep_above(true);
        window.set_position(WindowPosition::Center);
        window.connect_delete_event(|_, _| close());
        window.set_title(&self.settings.title);

        if let Some(context) = window.get_style_context() {
            let screen = context.get_screen().unwrap();
            StyleContext::add_provider_for_screen(
                &screen,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        let component = gtk::Box::new(gtk::Orientation::Horizontal, 10);

        if let Some(ref info) = self.settings.info {
            component.pack_start(&gtk::Label::new(info.as_str()), true, true, PADDING);
        }

        for &(fcode, ref button) in &self.settings.buttons {
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
}
