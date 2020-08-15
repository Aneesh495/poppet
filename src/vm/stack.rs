//! Funge Stack and Hierarchical Stack-Stack Architecture.
//!
//! Implements LIFO operand stacks with Funge-98 stack-stack framing semantics.
//! Supports block scopes, understack value transfers, and stack depth bounds.
//!
//! Copyright (c) 2026 Aneesh Krishna Parthasarathy
//! Licensed under the ISC License.

/// Individual LIFO operand stack.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FungeStack {
    pub values: Vec<i64>,
}

impl FungeStack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    /// Pushes a 64-bit integer value onto the stack.
    #[inline]
    pub fn push(&mut self, val: i64) {
        self.values.push(val);
    }

    /// Pops a value from the stack. An empty stack yields 0 per Befunge specification.
    #[inline]
    pub fn pop(&mut self) -> i64 {
        self.values.pop().unwrap_or(0)
    }

    /// Peeks at top of stack without consuming.
    #[inline]
    pub fn peek(&self) -> i64 {
        self.values.last().copied().unwrap_or(0)
    }

    /// Duplicates top of stack (:).
    #[inline]
    pub fn dup(&mut self) {
        let v = self.peek();
        self.push(v);
    }

    /// Swaps the top two values on the stack (\).
    #[inline]
    pub fn swap(&mut self) {
        let a = self.pop();
        let b = self.pop();
        self.push(a);
        self.push(b);
    }

    /// Drops the top value ($).
    #[inline]
    pub fn drop(&mut self) {
        let _ = self.pop();
    }

    /// Clears the stack (n).
    #[inline]
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Current depth of the stack.
    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true if stack is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// Befunge-98 hierarchical Stack-Stack structure.
#[derive(Debug, Clone, Default)]
pub struct FungeStackStack {
    /// Active operand stack stack.
    stacks: Vec<FungeStack>,
}

impl FungeStackStack {
    pub fn new() -> Self {
        Self {
            stacks: vec![FungeStack::new()],
        }
    }

    /// Returns a mutable reference to the active top stack.
    #[inline]
    pub fn active_mut(&mut self) -> &mut FungeStack {
        if self.stacks.is_empty() {
            self.stacks.push(FungeStack::new());
        }
        self.stacks.last_mut().unwrap()
    }

    /// Returns an immutable reference to the active top stack.
    #[inline]
    pub fn active(&self) -> &FungeStack {
        self.stacks.last().expect("Stack-stack must not be empty")
    }

    /// Begins a new block scope ({). Moves n items from current stack into new top stack.
    pub fn begin_block(&mut self, n: i64) {
        let mut new_stack = FungeStack::new();
        if n > 0 {
            let active = self.active_mut();
            let count = (n as usize).min(active.len());
            let split_idx = active.len() - count;
            let moved: Vec<i64> = active.values.drain(split_idx..).collect();
            new_stack.values = moved;
        } else if n < 0 {
            // Negative n pushes zero-initialized elements
            let count = (-n) as usize;
            new_stack.values = vec![0; count];
        }
        self.stacks.push(new_stack);
    }

    /// Ends block scope (}). Moves n items from active stack back down to underlying stack.
    pub fn end_block(&mut self, n: i64) {
        if self.stacks.len() <= 1 {
            // Cannot pop the base stack
            return;
        }
        let popping = self.stacks.pop().unwrap();
        let target = self.active_mut();

        if n > 0 {
            let count = (n as usize).min(popping.len());
            let split_idx = popping.len() - count;
            for val in &popping.values[split_idx..] {
                target.push(*val);
            }
        } else if n < 0 {
            let drop_count = (-n as usize).min(target.len());
            let new_len = target.len() - drop_count;
            target.values.truncate(new_len);
        }
    }

    /// Transfers items between active stack and understack (u).
    pub fn understack_transfer(&mut self, count: i64) {
        if self.stacks.len() < 2 {
            return;
        }
        let active_idx = self.stacks.len() - 1;
        let parent_idx = active_idx - 1;

        if count > 0 {
            for _ in 0..count {
                let v = self.stacks[parent_idx].pop();
                self.stacks[active_idx].push(v);
            }
        } else if count < 0 {
            for _ in 0..(-count) {
                let v = self.stacks[active_idx].pop();
                self.stacks[parent_idx].push(v);
            }
        }
    }

    /// Returns the number of stack frames.
    pub fn frame_count(&self) -> usize {
        self.stacks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funge_stack_basic_ops() {
        let mut s = FungeStack::new();
        assert_eq!(s.pop(), 0); // Empty yields 0

        s.push(10);
        s.push(20);
        assert_eq!(s.peek(), 20);
        assert_eq!(s.len(), 2);

        s.dup();
        assert_eq!(s.len(), 3);
        assert_eq!(s.pop(), 20);
        assert_eq!(s.pop(), 20);
        assert_eq!(s.pop(), 10);
        assert_eq!(s.pop(), 0);
    }

    #[test]
    fn test_funge_stack_swap() {
        let mut s = FungeStack::new();
        s.push(1);
        s.push(2);
        s.swap();
        assert_eq!(s.pop(), 1);
        assert_eq!(s.pop(), 2);
    }

    #[test]
    fn test_stack_stack_scoping() {
        let mut ss = FungeStackStack::new();
        ss.active_mut().push(100);
        ss.active_mut().push(200);
        ss.active_mut().push(300);

        // Begin block taking top 2 items
        ss.begin_block(2);
        assert_eq!(ss.frame_count(), 2);
        assert_eq!(ss.active().len(), 2);
        assert_eq!(ss.active().values, vec![200, 300]);

        // Mutate top frame
        ss.active_mut().push(400);

        // End block transferring top 1 item back down
        ss.end_block(1);
        assert_eq!(ss.frame_count(), 1);
        assert_eq!(ss.active().values, vec![100, 400]);
    }
}
