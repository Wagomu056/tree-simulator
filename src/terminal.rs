use std::io::{self, stdout};
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
                height: size.1,
            },
            _stdout: stdout().into_raw_mode().unwrap(),
        })
    }
    fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(x: u16, y: u16) {
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn draw_tree(&self, trees: &Vec<Vec<u8>>) {
        Self::cursor_position(1, 1);
        Self::clear_screen();

        for row in trees {
            let mut str: String = String::from("");
            for chara in row {
                str += &*chara.to_string();
            }
            print!("{}", str);
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}