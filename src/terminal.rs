use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(1),
            },
            _stdout: stdout().into_raw_mode().unwrap(),
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn draw_tree(&mut self, trees: &Vec<Vec<u8>>) {
        Self::cursor_hide();
        Self::cursor_position(1, 1);
        self.clear_screen();

        for row in trees {
            let mut str: String = String::from("");
            for chara in row {
                str += &*chara.to_string();
            }
            println!("{}\r", str);
        }
    }
    pub fn clear_screen(&self) {
        print!("{}", termion::clear::All);
    }
    pub fn check_finish(&self) -> bool {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                if key.unwrap() == Key::Ctrl('q') {
                    return true;
                }
                return false;
            }
        }
    }
    fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }
    fn cursor_position(x: u16, y: u16) {
        print!("{}", termion::cursor::Goto(x, y));
    }
}