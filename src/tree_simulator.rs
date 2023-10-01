use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use rand::{random, Rng};
use crate::tree_drawable::TreeDrawable;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
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
    const TREE_GROW_INTERVAL: u8 = 25;
    const TREE_INCREASE_INTERVAL_MIN: i8 = 15;
    const TREE_INCREASE_INTERVAL_MAX: i8 = 50;
    const FIRE_EXIST_TIME: i8 = 1;
    const TAKE_FIRE_RATIO: f64 = 0.01;
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
            sleep(Duration::from_millis(100));
        }
    }

    fn update(&mut self) {
        let all_positions = self.search_all_positions();
        let none_positions = &all_positions[&TreeType::None];
        let tree_positions = &all_positions[&TreeType::Tree];
        let fire_positions = &all_positions[&TreeType::Fire];

        let is_increase = self.update_increase_trees(tree_positions);
        let is_grown = self.update_grow_trees(none_positions);
        let is_spread = self.update_fire_spread(fire_positions);
        let is_fire = self.take_fire_at_random(tree_positions);

        if is_increase || is_grown || is_spread || is_fire {
            self.tree_drawable.draw_tree(&self.trees);
        }
    }

    fn update_grow_trees(&mut self, none_positions: &Vec<Pos>) -> bool {
        self.to_grow_count -= 1;
        if self.to_grow_count > 0 {
            return false;
        }

        self.to_grow_count = Self::TREE_GROW_INTERVAL;

        let rand
            = rand::thread_rng().gen_range(0..none_positions.len());
        self.set_tree_type(&none_positions[rand], TreeType::Tree);
        return true;
    }

    fn update_increase_trees(&mut self, tree_positions: &Vec<Pos>) -> bool {
        let mut is_dirty = false;
        for pos in tree_positions {
            if let Some(row) = self.increase_counts.get_mut(pos.y) {
                if let Some(increase_count) = row.get_mut(pos.x) {
                    *increase_count -= 1;
                    if *increase_count == 0 {
                        *increase_count = Self::get_increase_interval();

                        if let Some(around_pos) = self.search_around_none_pos(pos.x, pos.y) {
                            self.set_tree_type(&around_pos, TreeType::Tree);
                            is_dirty = true;
                        }
                    }
                }
            }
        }
        is_dirty
    }

    fn take_fire_at_random(&mut self, tree_positions: &Vec<Pos>) -> bool {
        let random: f64 = random();
        if random > Self::TAKE_FIRE_RATIO {
            return false;
        }

        if tree_positions.len() == 0 {
            return false;
        }

        let rnd_idx = rand::thread_rng().gen_range(0..tree_positions.len());
        self.set_tree_type(&tree_positions[rnd_idx], TreeType::Fire);
        return true;
    }

    fn update_fire_spread(&mut self, fire_positions: &Vec<Pos>) -> bool {
        let mut is_dirty = false;
        for pos in fire_positions {
            let count = self.increase_counts[pos.y][pos.x];
            if count == Self::FIRE_EXIST_TIME {
                let trees = self.search_around_tree_pos(&pos);
                for pos in trees {
                    self.set_tree_type(&pos, TreeType::Fire);
                    is_dirty = true;
                }
            } else if count == 0 {
                self.set_tree_type(&pos, TreeType::None);
                is_dirty = true;
            }

            self.increase_counts[pos.y][pos.x] -= 1;
        }
        is_dirty
    }

    fn search_around_none_pos(&self, x: usize, y: usize) -> Option<Pos> {
        let height = self.trees.len();
        let width = self.trees[0].len();

        let offset: u32 = random();
        for check_num in 0..4 {
            let num = (check_num + offset) % 4;
            match num {
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

    fn search_around_tree_pos(&self, pos: &Pos) -> Vec<Pos> {
        let x = pos.x;
        let y = pos.y;
        let height = self.trees.len();
        let width = self.trees[0].len();

        let mut tree_pos: Vec<Pos> = Vec::new();
        for check_num in 0..4 {
            match check_num {
                0 => {
                    if y == 0 { continue; }
                    if self.trees[y - 1][x] == TreeType::Tree {
                        tree_pos.push(Pos { x, y: y - 1 });
                    }
                }
                1 => {
                    if x >= width - 1 { continue; }
                    if self.trees[y][x + 1] == TreeType::Tree {
                        tree_pos.push(Pos { x: x + 1, y });
                    }
                }
                2 => {
                    if y >= height - 1 { continue; }
                    if self.trees[y + 1][x] == TreeType::Tree {
                        tree_pos.push(Pos { x, y: y + 1 });
                    }
                }
                3 => {
                    if x == 0 { continue; }
                    if self.trees[y][x - 1] == TreeType::Tree {
                        tree_pos.push(Pos { x: x - 1, y });
                    }
                }
                _ => {}
            }
        }
        tree_pos
    }

    fn search_all_positions(&self) -> HashMap<TreeType, Vec<Pos>> {
        let mut tree_positions: HashMap<TreeType, Vec<Pos>> = HashMap::new();
        tree_positions.insert(TreeType::None, Vec::new());
        tree_positions.insert(TreeType::Tree, Vec::new());
        tree_positions.insert(TreeType::Fire, Vec::new());

        for row_index in 0..self.trees.len() {
            for (column_index, element) in self.trees[row_index].iter().enumerate() {
                let pos = Pos { x: column_index, y: row_index };
                let positions = tree_positions.get_mut(element).unwrap();
                positions.push(pos);
            }
        }
        tree_positions
    }

    fn set_tree_type(&mut self, pos: &Pos, tree_type: TreeType) {
        if self.trees[pos.y][pos.x] == tree_type {
            return;
        }

        match tree_type {
            TreeType::None => {
                self.increase_counts[pos.y][pos.x] = -1;
            }
            TreeType::Tree => {
                self.increase_counts[pos.y][pos.x] = Self::get_increase_interval();
            }
            TreeType::Fire => {
                self.increase_counts[pos.y][pos.x] = Self::FIRE_EXIST_TIME;
            }
        }
        self.trees[pos.y][pos.x] = tree_type;
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
    const FIRE_EXIST_TIME: i8 = 1;
    const TAKE_FIRE_RATIO: f64 = 0.0;

    fn get_tree_type(&self, pos: &Pos) -> &TreeType {
        &self.trees[pos.y][pos.x]
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

    #[test]
    fn if_exist_fire_then_fire_spread() {
        let drawable = MockDrawable::default(
            Size { width: 10, height: 10 }
        );
        let mut sim = TreeSimulator::default(drawable);
        sim.set_tree_type(&Pos { x: 3, y: 2 }, TreeType::Tree);
        sim.set_tree_type(&Pos { x: 3, y: 3 }, TreeType::Fire);
        sim.set_tree_type(&Pos { x: 3, y: 4 }, TreeType::Tree);
        sim.set_tree_type(&Pos { x: 3, y: 5 }, TreeType::Tree);
        sim.set_tree_type(&Pos { x: 3, y: 6 }, TreeType::Tree);

        sim.set_tree_type(&Pos { x: 2, y: 3 }, TreeType::Tree);
        sim.set_tree_type(&Pos { x: 4, y: 3 }, TreeType::Tree);
        sim.set_tree_type(&Pos { x: 5, y: 3 }, TreeType::Tree);

        let all_positions = sim.search_all_positions();
        let fire_positions = &all_positions[&TreeType::Fire];
        sim.update_fire_spread(fire_positions);

        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 2 }), TreeType::Fire);
        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 3 }), TreeType::Fire);
        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 4 }), TreeType::Fire);
        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 5 }), TreeType::Tree);
        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 6 }), TreeType::Tree);

        assert_eq!(*sim.get_tree_type(&Pos { x: 2, y: 3 }), TreeType::Fire);
        assert_eq!(*sim.get_tree_type(&Pos { x: 4, y: 3 }), TreeType::Fire);
        assert_eq!(*sim.get_tree_type(&Pos { x: 5, y: 3 }), TreeType::Tree);

        let all_positions = sim.search_all_positions();
        let fire_positions = &all_positions[&TreeType::Fire];
        sim.update_fire_spread(fire_positions);

        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 2 }), TreeType::Fire);
        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 3 }), TreeType::None);
        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 4 }), TreeType::Fire);
        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 5 }), TreeType::Fire);
        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 6 }), TreeType::Tree);

        assert_eq!(*sim.get_tree_type(&Pos { x: 2, y: 3 }), TreeType::Fire);
        assert_eq!(*sim.get_tree_type(&Pos { x: 4, y: 3 }), TreeType::Fire);
        assert_eq!(*sim.get_tree_type(&Pos { x: 5, y: 3 }), TreeType::Fire);

        let all_positions = sim.search_all_positions();
        let fire_positions = &all_positions[&TreeType::Fire];
        sim.update_fire_spread(fire_positions);

        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 2 }), TreeType::None);
        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 3 }), TreeType::None);
        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 4 }), TreeType::None);
        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 5 }), TreeType::Fire);
        assert_eq!(*sim.get_tree_type(&Pos { x: 3, y: 6 }), TreeType::Fire);

        assert_eq!(*sim.get_tree_type(&Pos { x: 2, y: 3 }), TreeType::None);
        assert_eq!(*sim.get_tree_type(&Pos { x: 4, y: 3 }), TreeType::None);
        assert_eq!(*sim.get_tree_type(&Pos { x: 5, y: 3 }), TreeType::Fire);
    }
}