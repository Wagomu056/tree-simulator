use crate::tree_drawable::TreeDrawable;

pub struct TreeSimulator<T: TreeDrawable> {
    trees: Vec<Vec<u8>>,
    column_count: usize,
    row_count: usize,
    tree_drawable: T,
}

impl<T: TreeDrawable> TreeSimulator<T> {
    pub fn default(tree_drawable: T) -> Self {
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
            column_count,
            row_count,
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
    pub fn get_column_count(&self) -> usize {
        self.column_count
    }
    pub fn get_row_count(&self) -> usize {
        self.row_count
    }
}

#[cfg(test)]
mod tests {
    use crate::tree_drawable::Size;
    use super::*;

    struct MockDrawable {
        size: Size,
    }

    impl TreeDrawable for MockDrawable {
        fn size(&self) -> &Size { &self.size }
        fn draw_tree(&mut self, _trees: &Vec<Vec<u8>>) { todo!() }
    }

    impl MockDrawable {
        pub fn default(size: Size) -> Self {
            Self {
                size,
            }
        }
    }

    #[test]
    fn if_tree_size_eq_drawable_size() {
        let drawable = MockDrawable::default(
            Size {
                width: 3,
                height: 5,
            }
        );
        let simulator = TreeSimulator::default(drawable);
        assert_eq!(simulator.get_column_count(), 3);
        assert_eq!(simulator.get_row_count(), 5);
    }
}