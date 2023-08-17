mod context;
mod term;

use crate::term::RawTerm;

use std::io::{self, Read, Write};
use std::path;

#[derive(Debug)]
enum Event {
    Accept,
    DirDown,
    DirNext,
    DirPrev,
    DirUp,
    Quit,
}

pub struct Path<'a> {
    components: Vec<&'a str>, // Use Rc<str> or Box<str>
    selected: usize,
}

impl Path<'_> {
    pub fn up(&mut self) {
        if self.selected > 1 {
            self.selected -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.selected < self.components.len() {
            self.selected += 1;
        }
    }

    pub fn prefix(&self) -> String {
        self.components[..self.selected].join(path::MAIN_SEPARATOR_STR)
    }

    pub fn suffix(&self) -> String {
        self.components[self.selected..].join(path::MAIN_SEPARATOR_STR)
    }
}

impl<'a> From<&'a str> for Path<'a> {
    fn from(path: &'a str) -> Self {
        let components: Vec<&str> = path.split(path::MAIN_SEPARATOR_STR).collect();
        let selected = components.len() - 1;

        Path { components, selected }
    }
}

impl std::fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.components[..self.selected].join(path::MAIN_SEPARATOR_STR))
    }
}

trait Draw {
    fn draw(&self);
}

impl Draw for Path<'_> {
    fn draw(&self) {
        let selected = self.components[..self.selected].join(path::MAIN_SEPARATOR_STR);
        let deselected = self.components[self.selected..].join(path::MAIN_SEPARATOR_STR);

        eprint!("\x1b[1;36m{}/\x1b[0;30m{}\x1b[0m", selected, deselected);
    }
}


fn main() {
    let stdin_fd = 0;

    configure_cursor();

    with!(RawTerm::open(stdin_fd).unwrap(), {
        event_loop();
    });
}

fn configure_cursor() {
    print!("\x1b[5 q");
}

fn cursor_right(steps: u8) {
    print!("\x1b[{}C", steps);
}

fn event_loop() {
    let mut buf = [0; 8];
    let mut stdin = io::stdin();

    let cwd = std::env::current_dir().unwrap().to_string_lossy().to_string();
    let mut display = Path::from(cwd.as_str());

    loop {
        eprint!("\x1b[2K\r .. ");
        display.draw();
        eprint!("\r");
        cursor_right(20);
        let _ = io::stdout().flush();

        let _ = stdin.read(&mut buf);

        let event: Option<Event> = match buf[0] as char {
            '\n' => Some(Event::Accept),
            'h' => Some(Event::DirUp),
            'j' => Some(Event::DirNext),
            'k' => Some(Event::DirPrev),
            'l' => Some(Event::DirDown),
            'q' => Some(Event::Quit),
            _ => None,
        };

        match event {
            Some(Event::Accept) => {
                println!("\x1b[2K\r{}", display);
                break;
            },
            Some(Event::DirUp) => display.up(),
            Some(Event::DirDown) => display.down(),
            Some(Event::Quit) => break,
            _ => {}
        }
    }
}
