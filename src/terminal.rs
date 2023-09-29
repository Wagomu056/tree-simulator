use std::io::{self};
use crate::tree_drawable::{Size, TreeDrawable};

pub struct Terminal {
    size: Size,
}

impl TreeDrawable for Terminal {
    fn size(&self) -> &Size {
        &self.size
    }
    fn draw_tree(&mut self, trees: &Vec<Vec<u8>>) {
        Self::cursor_hide();
        Self::cursor_position(1, 1);

        for row in trees {
            Self::clear_current_line();

            let mut str: String = String::from("");
            for chara in row {
                str += &*chara.to_string();
            }
            println!("{}\r", str);
        }
    }
}

impl Terminal {
    pub fn default() -> Result<Self, io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(1),
            },
        })
    }
    fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }
    fn cursor_position(x: u16, y: u16) {
        print!("{}", termion::cursor::Goto(x, y));
    }
    fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
}