//! Funge-98 Core Interpreter and Virtual Machine Engine.
//!
//! Executes two-dimensional instruction topologies across Funge-Space with
//! cycle-accurate determinism, stack-stack memory management, and hardware MMIO integration.
//!
//! Copyright (c) 2026 Aneesh Krishna Parthasarathy
//! Licensed under the ISC License.

use super::space::{Coord, FungeSpace};
use super::stack::{FungeStack, FungeStackStack};

/// Trait for memory-mapped hardware peripheral hooks.
pub trait MmioHandler: Send + Sync {
    /// Intercepts read from Funge-Space coordinate. Returns Some(val) if handled.
    fn read(&mut self, x: i64, y: i64) -> Option<i64>;
    /// Intercepts write to Funge-Space coordinate. Returns true if handled.
    fn write(&mut self, x: i64, y: i64, val: i64) -> bool;
}

/// Execution state of the Funge Virtual Machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmState {
    Ready,
    Running,
    Halted,
    Breakpoint,
    Timeout,
    Error,
}

/// Main Funge-98 Virtual Machine.
pub struct Interpreter {
    pub space: FungeSpace,
    pub stack_stack: FungeStackStack,
    pub pos: Coord,
    pub delta: (i64, i64),
    pub storage_offset: Coord,
    pub string_mode: bool,
    pub state: VmState,
    pub cycle_count: u64,
    pub max_cycles: Option<u64>,
    pub output_buffer: Vec<u8>,
    pub input_buffer: Vec<u8>,
    pub input_cursor: usize,
    pub exit_code: i32,
    pub mmio_handler: Option<Box<dyn MmioHandler>>,
}

impl Interpreter {
    /// Creates a new Interpreter with a given grid dimension.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            space: FungeSpace::new(width, height),
            stack_stack: FungeStackStack::new(),
            pos: Coord::new(0, 0),
            delta: (1, 0),
            storage_offset: Coord::new(0, 0),
            string_mode: false,
            state: VmState::Ready,
            cycle_count: 0,
            max_cycles: None,
            output_buffer: Vec::new(),
            input_buffer: Vec::new(),
            input_cursor: 0,
            exit_code: 0,
            mmio_handler: None,
        }
    }

    /// Loads source code into Funge-Space at (0, 0) and resets instruction pointer.
    pub fn load_source(&mut self, source: &str) {
        self.space.load_source(source, 0, 0);
        self.reset();
    }

    /// Resets execution state to start at coordinate (0, 0) moving East (1, 0).
    pub fn reset(&mut self) {
        self.pos = Coord::new(0, 0);
        self.delta = (1, 0);
        self.storage_offset = Coord::new(0, 0);
        self.string_mode = false;
        self.state = VmState::Ready;
        self.cycle_count = 0;
        self.stack_stack = FungeStackStack::new();
        self.output_buffer.clear();
        self.input_cursor = 0;
        self.exit_code = 0;
    }

    /// Attaches an MMIO handler for peripheral hardware emulation.
    pub fn set_mmio_handler(&mut self, handler: Box<dyn MmioHandler>) {
        self.mmio_handler = Some(handler);
    }

    /// Sets an execution cycle budget to guard against infinite loops.
    pub fn set_max_cycles(&mut self, max: Option<u64>) {
        self.max_cycles = max;
    }

    /// Appends bytes to the standard input buffer.
    pub fn push_input(&mut self, bytes: &[u8]) {
        self.input_buffer.extend_from_slice(bytes);
    }

    /// Returns the active stack reference.
    pub fn active_stack(&self) -> &FungeStack {
        self.stack_stack.active()
    }

    /// Returns a mutable reference to the active operand stack.
    pub fn active_stack_mut(&mut self) -> &mut FungeStack {
        self.stack_stack.active_mut()
    }

    /// Steps a single instruction cycle. Returns current VM state.
    pub fn step(&mut self) -> VmState {
        if self.state == VmState::Halted || self.state == VmState::Timeout {
            return self.state;
        }

        if let Some(max) = self.max_cycles {
            if self.cycle_count >= max {
                self.state = VmState::Timeout;
                return self.state;
            }
        }

        self.cycle_count += 1;
        self.space.record_exec(self.pos.x, self.pos.y);

        let cell = self.space.get(self.pos.x, self.pos.y);

        if self.string_mode {
            if cell == '"' as i64 {
                self.string_mode = false;
            } else {
                self.active_stack_mut().push(cell);
            }
            self.advance_ip();
            return VmState::Running;
        }

        match cell as u8 {
            // Directional vectors
            b'>' => self.delta = (1, 0),
            b'<' => self.delta = (-1, 0),
            b'^' => self.delta = (0, -1),
            b'v' => self.delta = (0, 1),
            b'[' => {
                // Turn 90 degrees left: (dx, dy) -> (dy, -dx)
                self.delta = (self.delta.1, -self.delta.0);
            }
            b']' => {
                // Turn 90 degrees right: (dx, dy) -> (-dy, dx)
                self.delta = (-self.delta.1, self.delta.0);
            }
            b'r' => {
                // Reverse direction
                self.delta = (-self.delta.0, -self.delta.1);
            }
            b'?' => {
                // Deterministic pseudo-random cardinal cycle based on cycle count
                match self.cycle_count % 4 {
                    0 => self.delta = (1, 0),
                    1 => self.delta = (0, 1),
                    2 => self.delta = (-1, 0),
                    _ => self.delta = (0, -1),
                }
            }

            // Flow splitters
            b'_' => {
                let v = self.active_stack_mut().pop();
                if v == 0 {
                    self.delta = (1, 0);
                } else {
                    self.delta = (-1, 0);
                }
            }
            b'|' => {
                let v = self.active_stack_mut().pop();
                if v == 0 {
                    self.delta = (0, 1);
                } else {
                    self.delta = (0, -1);
                }
            }
            b'w' => {
                // Cardinal compare: pop b, pop a.
                // If a > b turn right; if a < b turn left; if a == b proceed straight.
                let b = self.active_stack_mut().pop();
                let a = self.active_stack_mut().pop();
                if a > b {
                    self.delta = (-self.delta.1, self.delta.0);
                } else if a < b {
                    self.delta = (self.delta.1, -self.delta.0);
                }
            }

            // String and character literals
            b'"' => self.string_mode = true,
            b'\'' => {
                // Fetch next cell literally and push its value, then skip over it
                self.advance_ip();
                let literal = self.space.get(self.pos.x, self.pos.y);
                self.active_stack_mut().push(literal);
            }

            // Numeric literals
            b'0'..=b'9' => {
                self.active_stack_mut().push((cell as u8 - b'0') as i64);
            }
            b'a'..=b'f' => {
                self.active_stack_mut().push((cell as u8 - b'a' + 10) as i64);
            }

            // Arithmetic and logic
            b'+' => {
                let b = self.active_stack_mut().pop();
                let a = self.active_stack_mut().pop();
                self.active_stack_mut().push(a.wrapping_add(b));
            }
            b'-' => {
                let b = self.active_stack_mut().pop();
                let a = self.active_stack_mut().pop();
                self.active_stack_mut().push(a.wrapping_sub(b));
            }
            b'*' => {
                let b = self.active_stack_mut().pop();
                let a = self.active_stack_mut().pop();
                self.active_stack_mut().push(a.wrapping_mul(b));
            }
            b'/' => {
                let b = self.active_stack_mut().pop();
                let a = self.active_stack_mut().pop();
                let res = if b == 0 { 0 } else { a.wrapping_div(b) };
                self.active_stack_mut().push(res);
            }
            b'%' => {
                let b = self.active_stack_mut().pop();
                let a = self.active_stack_mut().pop();
                let res = if b == 0 { 0 } else { a.wrapping_rem(b) };
                self.active_stack_mut().push(res);
            }
            b'!' => {
                let v = self.active_stack_mut().pop();
                self.active_stack_mut().push(if v == 0 { 1 } else { 0 });
            }
            b'`' => {
                let b = self.active_stack_mut().pop();
                let a = self.active_stack_mut().pop();
                self.active_stack_mut().push(if a > b { 1 } else { 0 });
            }

            // Stack manipulation
            b':' => self.active_stack_mut().dup(),
            b'\\' => self.active_stack_mut().swap(),
            b'$' => self.active_stack_mut().drop(),
            b'n' => self.active_stack_mut().clear(),

            // Stack-stack scoping
            b'{' => {
                let n = self.active_stack_mut().pop();
                self.stack_stack.begin_block(n);
            }
            b'}' => {
                let n = self.active_stack_mut().pop();
                self.stack_stack.end_block(n);
            }
            b'u' => {
                let count = self.active_stack_mut().pop();
                self.stack_stack.understack_transfer(count);
            }

            // Funge-Space memory access with MMIO hooks
            b'g' => {
                let y = self.active_stack_mut().pop() + self.storage_offset.y;
                let x = self.active_stack_mut().pop() + self.storage_offset.x;
                let val = if let Some(ref mut handler) = self.mmio_handler {
                    if let Some(intercepted) = handler.read(x, y) {
                        intercepted
                    } else {
                        self.space.get(x, y)
                    }
                } else {
                    self.space.get(x, y)
                };
                self.active_stack_mut().push(val);
            }
            b'p' => {
                let y = self.active_stack_mut().pop() + self.storage_offset.y;
                let x = self.active_stack_mut().pop() + self.storage_offset.x;
                let v = self.active_stack_mut().pop();
                let handled = if let Some(ref mut handler) = self.mmio_handler {
                    handler.write(x, y, v)
                } else {
                    false
                };
                if !handled {
                    self.space.put(x, y, v);
                }
            }

            // Control flow and skips
            b'#' => {
                // Bridge: skip next cell
                self.advance_ip();
            }
            b';' => {
                // Jump across comments until matching semicolon
                loop {
                    self.advance_ip();
                    let ch = self.space.get(self.pos.x, self.pos.y);
                    if ch == ';' as i64 {
                        break;
                    }
                }
            }
            b'z' => {
                // No-op
            }
            b'@' => {
                // Halt
                self.state = VmState::Halted;
                return self.state;
            }
            b'q' => {
                // Quit with exit code
                let code = self.active_stack_mut().pop();
                self.exit_code = code as i32;
                self.state = VmState::Halted;
                return self.state;
            }

            // I/O operations
            b',' => {
                let v = self.active_stack_mut().pop();
                self.output_buffer.push((v & 0xFF) as u8);
            }
            b'.' => {
                let v = self.active_stack_mut().pop();
                let s = format!("{} ", v);
                self.output_buffer.extend_from_slice(s.as_bytes());
            }
            b'~' => {
                // Read character from input buffer
                if self.input_cursor < self.input_buffer.len() {
                    let ch = self.input_buffer[self.input_cursor];
                    self.input_cursor += 1;
                    self.active_stack_mut().push(ch as i64);
                } else {
                    // Push -1 on EOF
                    self.active_stack_mut().push(-1);
                }
            }
            b'&' => {
                // Read integer from input buffer
                let remaining = &self.input_buffer[self.input_cursor..];
                let s = String::from_utf8_lossy(remaining);
                let trimmed = s.trim_start();
                let consumed_ws = s.len() - trimmed.len();
                self.input_cursor += consumed_ws;

                let mut digits = String::new();
                for ch in trimmed.chars() {
                    if ch.is_ascii_digit() || (digits.is_empty() && (ch == '-' || ch == '+')) {
                        digits.push(ch);
                    } else {
                        break;
                    }
                }

                if let Ok(num) = digits.parse::<i64>() {
                    self.input_cursor += digits.len();
                    self.active_stack_mut().push(num);
                } else {
                    self.active_stack_mut().push(0);
                }
            }

            // Ignored / space
            32 => {}

            _ => {
                // Unknown character treated as no-op per spec
            }
        }

        self.advance_ip();
        VmState::Running
    }

    /// Advances the instruction pointer according to current delta vector with toroidal wrap.
    #[inline]
    pub fn advance_ip(&mut self) {
        let (nx, ny) = self.space.wrap_coord(self.pos.x + self.delta.0, self.pos.y + self.delta.1);
        self.pos = Coord::new(nx, ny);
    }

    /// Executes instructions until halt, breakpoint, or timeout.
    pub fn run(&mut self) -> VmState {
        self.state = VmState::Running;
        while self.state == VmState::Running {
            self.step();
        }
        self.state
    }

    /// Returns output buffer decoded as UTF-8 string.
    pub fn get_output_string(&self) -> String {
        String::from_utf8_lossy(&self.output_buffer).into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_pipeline() {
        let mut vm = Interpreter::new(80, 25);
        vm.load_source("25*.@");
        vm.run();
        assert_eq!(vm.get_output_string().trim(), "10");
        assert_eq!(vm.state, VmState::Halted);
    }

    #[test]
    fn test_factorial_loop() {
        // Computes 5! = 120 in linear Befunge
        let mut vm = Interpreter::new(80, 25);
        vm.load_source("12345****.@");
        vm.run();
        assert_eq!(vm.get_output_string().trim(), "120");
        assert_eq!(vm.state, VmState::Halted);
    }

    #[test]
    fn test_string_mode() {
        let mut vm = Interpreter::new(80, 25);
        vm.load_source(r#""!dlroW ,olleH">:#,_@"#);
        vm.run();
        assert_eq!(vm.get_output_string(), "Hello, World!");
    }

    #[test]
    fn test_self_modifying_code() {
        // Overwrite cell at (5, 0) with '7', then execute it
        // In Befunge: v x y p -> pop y, pop x, pop v -> put(x, y, v)
        // Push 7 (val), push 5 (x), push 0 (y) -> p
        let source = ">750p0.@";
        let mut vm = Interpreter::new(80, 25);
        vm.load_source(source);
        vm.run();
        assert_eq!(vm.space.get(5, 0), 7);
        assert!(vm.space.mutation_count >= 9);
    }
}
