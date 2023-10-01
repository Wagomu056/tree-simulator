use std::io::{self};
use termion::{color};
use crate::tree_drawable::{Size, TreeDrawable};
use crate::tree_simulator::TreeType;

pub struct Terminal {
    size: Size,
}

impl TreeDrawable for Terminal {
    fn size(&self) -> &Size {
        &self.size
    }
    fn draw_tree(&self, trees: &Vec<Vec<TreeType>>) {
        print!("{}", color::Fg(color::Green));
        Self::cursor_hide();
        Self::cursor_position(1, 1);

        for row in trees {
            Self::clear_current_line();

            let mut str: String = String::from("");
            for chara in row {
                match chara {
                    TreeType::None => {
                        str += " ";
                    }
                    TreeType::Tree => {
                        str += "A";
                    }
                    TreeType::Fire => {
                        str += format!("{}*{}", color::Fg(color::Red), color::Fg(color::Green)).as_ref();
                    }
                }
            }
            println!("{}\r", str);
        }
        print!("{}", color::Fg(color::Reset));
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