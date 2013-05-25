extern mod extra;

use linearscan::{Allocator, Generator, GeneratorFunctions,
                 Config, Graph, KindHelper,
                 UseKind, UseAny, UseRegister, UseFixed};
use extra::json::ToJson;
use emulator::*;

#[path="../src/linearscan.rs"]
mod linearscan;
mod emulator;

impl KindHelper for Kind {
  fn is_call(&self) -> bool {
    match self {
      &Print => true,
      _ => false
    }
  }

  fn tmp_count(&self) -> uint {
    match self {
      &BranchIfBigger => 1,
      _ => 0
    }
  }

  fn use_kind(&self, i: uint) -> UseKind {
    match self {
      &BranchIfBigger if i == 0 => UseFixed(2),
      &JustUse => UseFixed(1),
      &Print => UseFixed(3),
      &Return => UseFixed(0),
      _ => UseAny
    }
  }

  fn result_kind(&self) -> Option<UseKind> {
    match self {
      &Return => None,
      &BranchIfBigger => None,
      &JustUse => None,
      _ => Some(UseRegister)
    }
  }
}

fn graph_test(expected: uint, body: &fn(b: &mut Graph<Kind>)) {
  let mut g = ~Graph::new::<Kind>();

  body(&mut *g);

  g.allocate(Config { register_count: 4 }).get();

  let mut emu = Emulator::new();
  assert!(emu.run(g) == expected);

  let writer = io::file_writer(&Path("./1.json"), [io::Create, io::Truncate]);
  match writer {
    Ok(writer) => writer.write_str(g.to_json().to_str()),
    Err(_) => ()
  };
}

#[test]
fn realword_example() {
  do graph_test(21) |g| {
    let phi = g.phi();

    let cond = g.empty_block();
    let left = g.empty_block();
    let after_left = g.empty_block();
    let right = g.empty_block();
    let ret = g.new_instr(Ten, ~[]);

    do g.block() |b| {
      b.make_root();

      b.add_existing(ret);
      let zero = b.add(Zero, ~[]);
      b.to_phi(zero, phi);
      b.goto(cond);
    };

    do g.with_block(cond) |b| {
      let ten = b.add(Ten, ~[]);
      b.add(JustUse, ~[phi]);
      b.add(BranchIfBigger, ~[phi, ten]);
      b.branch(right, left);
    };

    do g.with_block(left) |b| {
      let print_res = b.add(Print, ~[phi]);
      b.add(Increment, ~[print_res]);
      b.goto(after_left);
    };

    do g.with_block(after_left) |b| {
      let counter = b.add(Increment, ~[phi]);
      b.to_phi(counter, phi);
      b.goto(cond);
    };

    do g.with_block(right) |b| {
      let sum = b.add(Sum, ~[ret, phi]);
      b.add(Return, ~[sum]);
      b.end();
    };
  };
}
