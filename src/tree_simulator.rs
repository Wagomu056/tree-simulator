use std::thread::sleep;
use std::time::Duration;
use crate::tree_drawable::TreeDrawable;

#[derive(Clone)]
pub enum TreeType {
    None,
    Tree,
    Fire,
}

pub struct TreeSimulator<T: TreeDrawable> {
    trees: Vec<Vec<TreeType>>,
    tree_drawable: T,
}

impl<T: TreeDrawable> TreeSimulator<T> {
    pub fn default(tree_drawable: T) -> Self {
        let draw_size = tree_drawable.size();
        let column_count = draw_size.width as usize;
        let row_count = draw_size.height as usize;

        println!("initialize trees data [{}][{}]", column_count, row_count);
        let mut trees: Vec<Vec<TreeType>> = vec![vec![TreeType::None; column_count]; row_count];
        Self {
            trees,
            tree_drawable,
        }
    }
    pub fn run(&mut self) {
        loop {
            self.update();
            sleep(Duration::from_millis(500));
        }
    }

    fn update(&mut self) {
        for row in &mut self.trees {
            for ch in row {
                //*ch = (*ch + 1) % 10;
            }
        }

        self.tree_drawable.draw_tree(&self.trees);
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
        fn draw_tree(&self, _trees: &Vec<Vec<TreeType>>) {}
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