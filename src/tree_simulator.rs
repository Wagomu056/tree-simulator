use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use crate::tree_drawable::TreeDrawable;

#[derive(Clone, PartialEq)]
pub enum TreeType {
    None,
    Tree,
    Fire,
}

struct Pos {
    x: usize,
    y: usize,
}

pub struct TreeSimulator<T: TreeDrawable> {
    trees: Vec<Vec<TreeType>>,
    tree_drawable: T,
    to_grow_count: u8,
}

impl<T: TreeDrawable> TreeSimulator<T> {
    const TREE_GROW_INTERVAL: u8 = 5;
    const TREE_INCREASE_INTERVAL: u8 = 3;

    pub fn default(tree_drawable: T) -> Self {
        let draw_size = tree_drawable.size();
        let column_count = draw_size.width as usize;
        let row_count = draw_size.height as usize;

        println!("initialize trees data [{}][{}]", column_count, row_count);
        let trees: Vec<Vec<TreeType>> = vec![vec![TreeType::None; column_count]; row_count];
        Self {
            trees,
            tree_drawable,
            to_grow_count: Self::TREE_GROW_INTERVAL,
        }
    }
    pub fn run(&mut self) {
        loop {
            self.update();
            sleep(Duration::from_millis(500));
        }
    }

    fn update(&mut self) {
        self.update_grow_trees();
        self.tree_drawable.draw_tree(&self.trees);
    }

    fn update_grow_trees(&mut self) {
        self.to_grow_count -= 1;
        if self.to_grow_count > 0 {
            return;
        }

        self.to_grow_count = Self::TREE_GROW_INTERVAL;

        let mut nones: Vec<Pos> = Vec::new();
        for row_index in 0..self.trees.len() {
            for (column_index, element) in self.trees[row_index].iter().enumerate() {
                if *element == TreeType::None {
                    nones.push(
                        Pos {
                            x: column_index,
                            y: row_index,
                        }
                    );
                }
            }
        }

        let rand
            = rand::thread_rng().gen_range(0..nones.len());
        let pos
            = Pos{ x: nones[rand].x, y: nones[rand].y };
        self.trees[pos.y][pos.x] = TreeType::Tree;
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

    fn get_tree_count(sim: &TreeSimulator<MockDrawable>) -> i32 {
        let mut tree_count = 0;
        for row in &sim.trees {
            for elem in row {
                if *elem == TreeType::Tree {
                    tree_count += 1;
                }
            }
        }
        tree_count
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
    #[test]
    fn if_update_few_time_then_one_tree_grow() {
        let drawable = MockDrawable::default(
            Size { width: 10, height: 10, }
        );
        let mut sim = TreeSimulator::default(drawable);

        // first, there is no tree
        let tree_count = get_tree_count(&sim);
        assert_eq!(tree_count, 0);

        // update few times
        for _num in 0..TreeSimulator::<MockDrawable>::TREE_GROW_INTERVAL {
            sim.update();
        }

        // then one tree grown
        let tree_count = get_tree_count(&sim);
        assert_eq!(tree_count, 1);
    }
}