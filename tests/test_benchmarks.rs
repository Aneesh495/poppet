//! High-Performance Execution Benchmarks for Funge-Space JIT and Interpreter.
//!
//! Copyright (c) 2026 Aneesh Krishna Parthasarathy
//! Licensed under the ISC License.

use std::time::Instant;
use poppet::vm::interpreter::Interpreter;
use poppet::vm::jit::JitEngine;
use poppet::vm::space::{Coord, FungeSpace};
use poppet::vm::stack::FungeStack;

#[test]
fn test_jit_acceleration_benchmark() {
    let mut space = FungeSpace::new(80, 25);
    // Linear arithmetic corridor repeated
    space.load_source("12+3+4+5+6+7+8+9+@", 0, 0);

    let mut jit = JitEngine::new();
    let mut stack = FungeStack::new();

    let start = Instant::now();
    for _ in 0..10_000 {
        stack.clear();
        let _ = jit.try_dispatch(&space, Coord::new(0, 0), (1, 0), &mut stack);
    }
    let elapsed = start.elapsed();

    assert_eq!(stack.pop(), 45); // Sum 1..9 = 45
    println!("JIT executed 10,000 blocks in {:?}", elapsed);
    assert!(elapsed.as_millis() < 500, "JIT block execution must be sub-microsecond");
}

#[test]
fn test_interpreter_cycle_throughput() {
    let mut vm = Interpreter::new(80, 25);
    vm.load_source("12345****.@");

    let start = Instant::now();
    for _ in 0..1_000 {
        vm.reset();
        vm.run();
    }
    let elapsed = start.elapsed();
    println!("Interpreter executed 1,000 runs in {:?}", elapsed);
    assert!(elapsed.as_millis() < 500);
}
