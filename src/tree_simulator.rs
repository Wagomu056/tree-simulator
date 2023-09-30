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
    increase_counts: Vec<Vec<i8>>,
    tree_drawable: T,
    to_grow_count: u8,
}

// release parameters
#[cfg(not(test))]
impl<T: TreeDrawable> TreeSimulator<T> {
    const TREE_GROW_INTERVAL: u8 = 5;
    const TREE_INCREASE_INTERVAL_MIN: i8 = 3;
    const TREE_INCREASE_INTERVAL_MAX: i8 = 10;
}

impl<T: TreeDrawable> TreeSimulator<T> {
    pub fn default(tree_drawable: T) -> Self {
        let draw_size = tree_drawable.size();
        let column_count = draw_size.width as usize;
        let row_count = draw_size.height as usize;

        println!("initialize trees data [{}][{}]", column_count, row_count);
        let trees: Vec<Vec<TreeType>> = vec![vec![TreeType::None; column_count]; row_count];
        let increase_counts: Vec<Vec<i8>> = vec![vec![-1; column_count]; row_count];
        Self {
            trees,
            increase_counts,
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
        self.update_increase_trees();
        self.update_grow_trees();
        self.tree_drawable.draw_tree(&self.trees);
    }

    fn update_grow_trees(&mut self) {
        self.to_grow_count -= 1;
        if self.to_grow_count > 0 {
            return;
        }

        self.to_grow_count = Self::TREE_GROW_INTERVAL;

        let nones: Vec<Pos> = self.search_positions(TreeType::None);
        let rand
            = rand::thread_rng().gen_range(0..nones.len());
        self.set_tree(nones[rand].x, nones[rand].y);
    }

    fn update_increase_trees(&mut self) {
        let tree_positions = self.search_positions(TreeType::Tree);
        for pos in tree_positions {
            if let Some(row) = self.increase_counts.get_mut(pos.y) {
                if let Some(increase_count) = row.get_mut(pos.x) {
                    *increase_count -= 1;
                    if *increase_count == 0 {
                        *increase_count = Self::get_increase_interval();

                        if let Some(around_pos) = self.search_around_none_pos(pos.x, pos.y) {
                            self.set_tree(around_pos.x, around_pos.y);
                        }
                    }
                }
            }
        }
    }

    fn search_around_none_pos(&self, x: usize, y: usize) -> Option<Pos> {
        let height = self.trees.len();
        let width = self.trees[0].len();

        for check_num in 0..4 {
            match check_num {
                0 => {
                    if y == 0 { continue; }
                    if self.trees[y - 1][x] == TreeType::None {
                        return Some(Pos { x, y: y - 1 });
                    }
                }
                1 => {
                    if x >= width - 1 { continue; }
                    if self.trees[y][x + 1] == TreeType::None {
                        return Some(Pos { x: x + 1, y });
                    }
                }
                2 => {
                    if y >= height - 1 { continue; }
                    if self.trees[y + 1][x] == TreeType::None {
                        return Some(Pos { x, y: y + 1 });
                    }
                }
                3 => {
                    if x == 0 { continue; }
                    if self.trees[y][x - 1] == TreeType::None {
                        return Some(Pos { x: x - 1, y });
                    }
                }
                _ => { return None; }
            }
        }
        None
    }

    fn search_positions(&self, target_type: TreeType) -> Vec<Pos> {
        let mut positions: Vec<Pos> = Vec::new();
        for row_index in 0..self.trees.len() {
            for (column_index, element) in self.trees[row_index].iter().enumerate() {
                if *element == target_type {
                    positions.push(
                        Pos {
                            x: column_index,
                            y: row_index,
                        }
                    );
                }
            }
        }
        positions
    }

    fn set_tree(&mut self, x: usize, y: usize) {
        self.trees[y][x] = TreeType::Tree;
        self.increase_counts[y][x] = Self::get_increase_interval();
    }

    fn get_increase_interval() -> i8 {
        rand::thread_rng().gen_range(
            Self::TREE_INCREASE_INTERVAL_MIN..=Self::TREE_INCREASE_INTERVAL_MAX)
    }
}

// in test parameters
#[cfg(test)]
impl<T: TreeDrawable> TreeSimulator<T> {
    const TREE_GROW_INTERVAL: u8 = 10;
    const TREE_INCREASE_INTERVAL_MIN: i8 = 1;
    const TREE_INCREASE_INTERVAL_MAX: i8 = 1;
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

    // targetがoriginの周囲に存在しているか？
    fn is_exist_around(origin: &Pos, target: &Pos) -> bool {
        // eq origin
        if origin.x == target.x && origin.y == target.y {
            return true;
        }
        // up
        if origin.y > 0 {
            if origin.y - 1 == target.y
                && origin.x == target.x {
                return true;
            }
        }
        // down
        if origin.y < usize::MAX {
            if origin.y + 1 == target.y
                && origin.x == target.x {
                return true;
            }
        }
        // right
        if origin.x > 0 {
            if origin.x - 1 == target.x
                && origin.y == target.y {
                return true;
            }
        }
        // left
        if origin.x < usize::MAX {
            if origin.x + 1 == target.x
                && origin.y == target.y {
                return true;
            }
        }
        false
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
            Size { width: 10, height: 10 }
        );
        let mut sim = TreeSimulator::default(drawable);

        // first, there is no tree
        let trees = sim.search_positions(TreeType::Tree);
        assert_eq!(trees.len(), 0);

        // update few times
        for _num in 0..TreeSimulator::<MockDrawable>::TREE_GROW_INTERVAL {
            sim.update();
        }

        // then one tree grown
        let trees = sim.search_positions(TreeType::Tree);
        assert_eq!(trees.len(), 1);
    }

    #[test]
    fn if_update_few_times_then_grow_and_increase_trees() {
        let drawable = MockDrawable::default(
            Size { width: 10, height: 10 }
        );
        let mut sim = TreeSimulator::default(drawable);

        // update few times
        for _num in 0..TreeSimulator::<MockDrawable>::TREE_GROW_INTERVAL {
            sim.update();
        }

        // then one tree grown
        let trees = sim.search_positions(TreeType::Tree);
        assert_eq!(trees.len(), 1);
        // 最初に木が生えた位置を保存する
        let pos_origin = &trees[0];

        sim.update();

        let trees = sim.search_positions(TreeType::Tree);
        assert_eq!(trees.len(), 2);

        // 最初の木から周囲のどこかに生えているはず
        for pos in trees {
            assert_eq!(is_exist_around(pos_origin, &pos), true);
        }
    }
}