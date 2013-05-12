use linearscan::graph::{Graph, KindHelper};
use linearscan::flatten::Flatten;
use linearscan::liveness::Liveness;

pub struct Config {
  register_count: uint
}

pub trait Allocator {
  // Allocate registers
  pub fn allocate(&mut self, config: Config);
}

impl<K: KindHelper> Allocator for Graph<K> {
  pub fn allocate(&mut self, config: Config) {
    let list = self.flatten();
    self.build_liveranges(list);
    io::println(fmt!("%?", list));
  }
}
