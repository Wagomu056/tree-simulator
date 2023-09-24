mod tree_simulator;
mod terminal;
mod tree_drawable;

pub use terminal::Terminal;
use crate::tree_simulator::TreeSimulator;

fn main() {
    let tree_drawable = Terminal::default().expect("Failed to initialize");
    TreeSimulator::default(tree_drawable).run();
}
