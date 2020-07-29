//! Funge-98 Core Interpreter.

use super::space::FungeSpace;
use super::stack::FungeStack;

pub struct Interpreter {
    pub space: FungeSpace,
    pub stack: FungeStack,
}
