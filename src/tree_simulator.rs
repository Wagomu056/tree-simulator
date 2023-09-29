use std::thread::sleep;
use std::time::Duration;
use crate::tree_drawable::TreeDrawable;

pub struct TreeSimulator<T: TreeDrawable> {
    trees: Vec<Vec<u8>>,
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
            tree_drawable,
        }
    }
    pub fn run(&mut self) {
        loop {
            self.tree_drawable.draw_tree(&self.trees);

            for row in &mut self.trees {
                for ch in row {
                    *ch = (*ch + 1) % 10;
                }
            }

            sleep(Duration::from_millis(500));
        }
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
        fn draw_tree(&self, _trees: &Vec<Vec<u8>>) { todo!() }
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
        // width
        assert_eq!(simulator.trees[0].len(), 3);
        // height
        assert_eq!(simulator.trees.len(), 5);
    }
}