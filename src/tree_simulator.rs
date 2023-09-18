use crate::Terminal;

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct TreeSimulator {
    trees: Vec<Vec<u8>>,
    _tree_drawable: Terminal,
}

impl TreeSimulator {
    pub fn default() -> Self {
        let tree_drawable = Terminal::default().expect("Failed to initialize");

        let draw_size = tree_drawable.size();
        let column_count = draw_size.width as usize;
        let row_count = draw_size.height as usize;

        println!("initialize trees data [{}][{}]", column_count, row_count);
        let trees: Vec<Vec<u8>> = vec![vec![0; column_count]; row_count];
        Self {
            trees,
            _tree_drawable: tree_drawable,
        }
    }

    pub fn get_trees(&self) -> &Vec<Vec<u8>> {
        &self.trees
    }
}

