use gtk::*;
use sourceview::*;

pub struct Source {
    pub container: ScrolledWindow,
    pub view: View,
    pub buff: Buffer,
}

impl Source {
    pub fn new() -> Source {
        // Create the SourceView for the eidtor on the left pane
        let buff = Buffer::new(None);
        let view = View::new_with_buffer(&buff);
        let container = ScrolledWindow::new(None, None);
        container.add(&view);

        Source {
            buff,
            view,
            container,
        }
    }
}

fn configure_source_view(view: &View, buff: &Buffer) {
    view.set_show_line_numbers(true);
    view.set_monospace(true);
    view.set_insert_spaces_instead_of_tabs(true);
    view.set_indent_width(4);
    view.set_smart_backspace(true);
    view.set_right_margin(100);
    view.set_left_margin(10);
    view.set_show_right_margin(true);
    view.set_background_pattern(BackgroundPatternType::Grid);
}
