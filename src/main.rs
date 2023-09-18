mod tree_simulator;
mod terminal;

pub use tree_simulator::Position;
pub use terminal::Terminal;
use crate::tree_simulator::TreeSimulator;

fn main() {
    let simulator = TreeSimulator::default();
    let _trees = simulator.get_trees();
    println!("Hello, world!");
}
