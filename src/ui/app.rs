use gtk::*;

pub struct App {
    window: Window,
}

impl App {
    pub fn new() -> App {
        // Initialize GTK before proceeding
        if gtk::init().is_err() {
            // eprintln!("failed to initialize GTK Application");
            // std::process::exit(1);

            // Just panic instead. `process::exit` does not call `Drop`
            panic!("failed to initialize GTK Application");
        }

        // Create a new top level window.
        let window = Window::new(WindowType::Toplevel);
        // Create a headerbar and it's associated content.

        // window.set_titlebar

        // Set the title of the window
        window.set_title("Markdown Editor");
        // Set the window manager class.panic!
        window.set_role("md-editor");
        // The icon the app will display
        window.set_default_size(800, 600);
        Window::set_default_icon_name("iconname");
        // Add the content to the window
        // window.add

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App { window }
    }
}
