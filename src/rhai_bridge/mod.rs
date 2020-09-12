//! Rhai Embedded Scripting Host for SCADA Procedures and Automated Testing.
//!
//! Exposes a high-level Rhai API allowing operators to author dynamic test campaigns,
//! fault injection scenarios, and automated commissioning sequences against the Befunge core.
//!
//! Copyright (c) 2026 Aneesh Krishna Parthasarathy
//! Licensed under the ISC License.

use rhai::{CustomType, Engine, Scope, TypeBuilder};
use crate::hal::HydraulicHal;
use crate::safety::SafetyMonitor;
use crate::vm::interpreter::Interpreter;

/// High-level executive combining Befunge VM, Hydraulic HAL, and Safety Supervisor.
#[derive(Clone)]
pub struct PoppetSystem {
    pub hal: HydraulicHal,
    pub safety: SafetyMonitor,
}

impl Default for PoppetSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PoppetSystem {
    pub fn new() -> Self {
        Self {
            hal: HydraulicHal::new(),
            safety: SafetyMonitor::default(),
        }
    }

    pub fn set_pressure_a(&mut self, bar_x10: i64) {
        self.hal.sensor_a = bar_x10;
    }

    pub fn set_pressure_b(&mut self, bar_x10: i64) {
        self.hal.sensor_b = bar_x10;
    }

    pub fn set_pressure_c(&mut self, bar_x10: i64) {
        self.hal.sensor_c = bar_x10;
    }

    pub fn get_pressure_a(&mut self) -> i64 {
        self.hal.sensor_a
    }

    pub fn get_pressure_b(&mut self) -> i64 {
        self.hal.sensor_b
    }

    pub fn get_pressure_c(&mut self) -> i64 {
        self.hal.sensor_c
    }

    pub fn inject_surge(&mut self, peak_bar_x10: i64) {
        self.hal.inject_overpressure(peak_bar_x10);
    }

    pub fn is_tripped(&mut self) -> bool {
        self.hal.trip_coil_deenergized
    }

    pub fn get_valve_1_stroke(&mut self) -> i64 {
        self.hal.valve_1.stroke_mille
    }

    pub fn get_valve_2_stroke(&mut self) -> i64 {
        self.hal.valve_2.stroke_mille
    }

    pub fn get_flow_rate(&mut self) -> i64 {
        self.hal.flow_rate
    }

    pub fn tick_physics(&mut self, dt_ms: f64) {
        self.hal.tick(dt_ms);
        self.safety.supervise(&mut self.hal);
    }

    pub fn execute_befunge_kernel(&mut self, kernel_source: String) -> i64 {
        let mut vm = Interpreter::new(128, 64);
        vm.load_source(&kernel_source);
        vm.set_max_cycles(Some(5000));
        let res = vm.run();
        if res == crate::vm::interpreter::VmState::Halted {
            vm.active_stack().peek()
        } else {
            -1
        }
    }
}

impl CustomType for PoppetSystem {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("PoppetSystem")
            .with_fn("new_system", PoppetSystem::new)
            .with_fn("set_pressure_a", PoppetSystem::set_pressure_a)
            .with_fn("set_pressure_b", PoppetSystem::set_pressure_b)
            .with_fn("set_pressure_c", PoppetSystem::set_pressure_c)
            .with_fn("get_pressure_a", PoppetSystem::get_pressure_a)
            .with_fn("get_pressure_b", PoppetSystem::get_pressure_b)
            .with_fn("get_pressure_c", PoppetSystem::get_pressure_c)
            .with_fn("inject_surge", PoppetSystem::inject_surge)
            .with_fn("is_tripped", PoppetSystem::is_tripped)
            .with_fn("get_valve_1_stroke", PoppetSystem::get_valve_1_stroke)
            .with_fn("get_valve_2_stroke", PoppetSystem::get_valve_2_stroke)
            .with_fn("get_flow_rate", PoppetSystem::get_flow_rate)
            .with_fn("tick_physics", PoppetSystem::tick_physics)
            .with_fn("execute_befunge_kernel", PoppetSystem::execute_befunge_kernel);
    }
}

/// Scripting host engine for Rhai automation.
pub struct RhaiEngine {
    engine: Engine,
}

impl Default for RhaiEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RhaiEngine {
    pub fn new() -> Self {
        let mut engine = Engine::new();
        engine.build_type::<PoppetSystem>();
        Self { engine }
    }

    /// Evaluates a Rhai script string against a shared PoppetSystem instance.
    pub fn eval_with_system(&self, script: &str, system: &mut PoppetSystem) -> Result<(), Box<rhai::EvalAltResult>> {
        let mut scope = Scope::new();
        scope.push("sys", system.clone());
        self.engine.run_with_scope(&mut scope, script)?;
        *system = scope.get_value::<PoppetSystem>("sys").unwrap();
        Ok(())
    }

    /// Evaluates an independent Rhai script string.
    pub fn eval(&self, script: &str) -> Result<rhai::Dynamic, Box<rhai::EvalAltResult>> {
        self.engine.eval(script)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rhai_system_control() {
        let rhai = RhaiEngine::new();
        let mut sys = PoppetSystem::new();

        let script = r#"
            sys.set_pressure_a(3600);
            sys.set_pressure_b(3650);
            sys.set_pressure_c(2500);
            sys.tick_physics(10.0);
        "#;

        assert!(rhai.eval_with_system(script, &mut sys).is_ok());
        assert!(sys.is_tripped());
    }

    #[test]
    fn test_rhai_execute_befunge() {
        let rhai = RhaiEngine::new();
        let mut sys = PoppetSystem::new();

        let script = r#"
            let befunge_code = "78*@";
            let result = sys.execute_befunge_kernel(befunge_code);
            if result != 56 {
                throw "Mismatch result: " + result;
            }
        "#;

        assert!(rhai.eval_with_system(script, &mut sys).is_ok());
    }
}
