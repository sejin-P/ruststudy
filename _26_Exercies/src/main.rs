// TODO: remove this when you're done with your implementation.
use std::fmt::Write;

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
    label_wrapper: String,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback: callback,
            label_wrapper: "| ".into(),
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
    width: usize,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
            width: title.len() + 2,
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        if self.width < widget.width() {
            self.width = widget.width()
        }
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}


impl Widget for Label {
    fn width(&self) -> usize {
        self.label.len()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        writeln!(buffer, "{}", &self.label).unwrap()
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + self.label_wrapper.len() * 2
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        writeln!(buffer, "{} {} {}",
        &self.label_wrapper,
        &self.label.label,
        &self.label_wrapper.chars().rev().collect::<String>()
        ).unwrap()
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        self.width
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let mut str_buffer = String::new();
        for widget in &self.widgets {
            writeln!(&mut str_buffer, "").unwrap();
            widget.draw_into(&mut str_buffer);
        }

        writeln!(buffer, "{}", "=".repeat(self.width()));
        writeln!(buffer, "{}", self.title);
        writeln!(buffer, "{}", "=".repeat(self.width()));
        writeln!(buffer, "{}", str_buffer);
    }
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}