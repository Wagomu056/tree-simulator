
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub trait TreeDrawable {
    fn size(&self) -> &Size;
    fn draw_tree(&mut self, trees: &Vec<Vec<u8>>);
}