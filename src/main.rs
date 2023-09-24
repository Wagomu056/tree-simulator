mod tree_simulator;
mod terminal;

pub use terminal::Terminal;
use crate::tree_simulator::TreeSimulator;

fn main() {
    TreeSimulator::default().run();
}
