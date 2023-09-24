use crate::Terminal;

pub struct TreeSimulator {
    trees: Vec<Vec<u8>>,
    tree_drawable: Terminal,
}

impl TreeSimulator {
    pub fn default() -> Self {
        let tree_drawable = Terminal::default().expect("Failed to initialize");

        let draw_size = tree_drawable.size();
        let column_count = draw_size.width as usize;
        let row_count = draw_size.height as usize;

        println!("initialize trees data [{}][{}]", column_count, row_count);
        let mut trees: Vec<Vec<u8>> = vec![vec![0; column_count]; row_count];
        for row in &mut trees {
            for chara in row {
                *chara = 1;
            }
        }
        Self {
            trees,
            tree_drawable,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.tree_drawable.draw_tree(&self.trees);

            if self.tree_drawable.check_finish() {
                break;
            }
        }

        self.tree_drawable.clear_screen();
    }
}

