//! Interactive TUI Debugger, Breakpoint Engine, and Spatial Heatmap Profiler.
//!
//! Provides step-by-step debugging, coordinate breakpoints, stack introspection,
//! and 2D Funge-Space visual grid inspection.
//!
//! Copyright (c) 2026 Aneesh Krishna Parthasarathy
//! Licensed under the ISC License.

use std::collections::HashSet;
use super::interpreter::Interpreter;
use super::space::Coord;

/// Debugger breakpoint specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Breakpoint {
    pub coord: Coord,
}

/// Execution trace frame for time-travel and post-mortem analysis.
#[derive(Debug, Clone)]
pub struct TraceFrame {
    pub cycle: u64,
    pub pos: Coord,
    pub delta: (i64, i64),
    pub opcode: i64,
    pub stack_top: i64,
    pub stack_depth: usize,
}

/// Interactive and headless debugger manager.
#[derive(Debug)]
pub struct Debugger {
    pub breakpoints: HashSet<Coord>,
    pub trace_history: Vec<TraceFrame>,
    pub max_history: usize,
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl Debugger {
    pub fn new(max_history: usize) -> Self {
        Self {
            breakpoints: HashSet::new(),
            trace_history: Vec::with_capacity(max_history),
            max_history,
        }
    }

    /// Adds a coordinate breakpoint at (x, y).
    pub fn add_breakpoint(&mut self, x: i64, y: i64) {
        self.breakpoints.insert(Coord::new(x, y));
    }

    /// Removes a breakpoint at (x, y).
    pub fn remove_breakpoint(&mut self, x: i64, y: i64) {
        self.breakpoints.remove(&Coord::new(x, y));
    }

    /// Records current execution state into trace history.
    pub fn record_step(&mut self, vm: &Interpreter) {
        let frame = TraceFrame {
            cycle: vm.cycle_count,
            pos: vm.pos,
            delta: vm.delta,
            opcode: vm.space.get(vm.pos.x, vm.pos.y),
            stack_top: vm.active_stack().peek(),
            stack_depth: vm.active_stack().len(),
        };

        if self.trace_history.len() >= self.max_history {
            self.trace_history.remove(0);
        }
        self.trace_history.push(frame);
    }

    /// Steps the VM once, recording trace and checking for breakpoints.
    pub fn step_and_check(&mut self, vm: &mut Interpreter) -> bool {
        self.record_step(vm);
        vm.step();
        self.breakpoints.contains(&vm.pos)
    }

    /// Renders ASCII representation of Funge-Space viewport centered around current IP.
    pub fn render_viewport(&self, vm: &Interpreter, radius_x: i64, radius_y: i64) -> String {
        let mut out = String::new();
        let cx = vm.pos.x;
        let cy = vm.pos.y;

        out.push_str(&format!(
            "--- Funge-Space Viewport [IP: ({}, {}), Delta: ({}, {}), Cycle: {}] ---\n",
            cx, cy, vm.delta.0, vm.delta.1, vm.cycle_count
        ));

        for y in (cy - radius_y)..=(cy + radius_y) {
            for x in (cx - radius_x)..=(cx + radius_x) {
                if x == cx && y == cy {
                    // Instruction pointer cursor glyph
                    let glyph = match vm.delta {
                        (1, 0) => '>',
                        (-1, 0) => '<',
                        (0, 1) => 'v',
                        (0, -1) => '^',
                        _ => '*',
                    };
                    out.push('[');
                    out.push(glyph);
                    out.push(']');
                } else {
                    let cell = vm.space.get(x, y);
                    let ch = if (32..=126).contains(&cell) {
                        (cell as u8) as char
                    } else {
                        '.'
                    };
                    out.push(' ');
                    out.push(ch);
                    out.push(' ');
                }
            }
            out.push('\n');
        }

        out.push_str(&format!(
            "Stack (depth {}): {:?}\n",
            vm.active_stack().len(),
            vm.active_stack().values
        ));

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debugger_breakpoints() {
        let mut vm = Interpreter::new(80, 25);
        vm.load_source(">25*4+@");

        let mut dbg = Debugger::new(50);
        dbg.add_breakpoint(3, 0); // Break at '4'

        let mut hit = false;
        while vm.state == super::super::interpreter::VmState::Ready || vm.state == super::super::interpreter::VmState::Running {
            if dbg.step_and_check(&mut vm) {
                hit = true;
                break;
            }
        }

        assert!(hit);
        assert_eq!(vm.pos, Coord::new(3, 0));
        assert_eq!(dbg.trace_history.len(), 3);
    }

    #[test]
    fn test_viewport_rendering() {
        let mut vm = Interpreter::new(80, 25);
        vm.load_source(">12+@");
        let dbg = Debugger::new(10);
        let view = dbg.render_viewport(&vm, 2, 1);
        assert!(view.contains("Funge-Space Viewport"));
        assert!(view.contains("[>]"));
    }
}
