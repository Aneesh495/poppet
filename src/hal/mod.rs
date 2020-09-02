//! Hardware Abstraction Layer for Subsea Electro-Hydraulic Poppet Manifolds.
//!
//! Models physical characteristics of high-pressure hydraulic accumulators,
//! bi-stable poppet valves, 4-20mA pressure transmitters, and fluid transient dynamics.
//! Implements MmioHandler to map Funge-Space memory regions directly to physical registers.
//!
//! Copyright (c) 2026 Aneesh Krishna Parthasarathy
//! Licensed under the ISC License.

use crate::vm::interpreter::MmioHandler;

/// Standard MMIO register addresses in Funge-Space (Y = 100 row).
pub const REG_TRANSDUCER_A: i64 = 0;
pub const REG_TRANSDUCER_B: i64 = 1;
pub const REG_TRANSDUCER_C: i64 = 2;
pub const REG_ACCUMULATOR_PRES: i64 = 3;
pub const REG_VALVE_1_CMD: i64 = 4;
pub const REG_VALVE_2_CMD: i64 = 5;
pub const REG_TRIP_COIL: i64 = 6;
pub const REG_ALARM_FLAGS: i64 = 7;
pub const REG_MONOTONIC_TIME: i64 = 8;
pub const REG_WATCHDOG_KICK: i64 = 9;
pub const REG_ESD_LEVEL: i64 = 10;
pub const REG_FLOW_RATE: i64 = 11;

/// Poppet valve physical state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValveState {
    Closed,
    Opening,
    Open,
    Closing,
    FaultStuck,
}

/// Simulated high-pressure hydraulic cartridge poppet valve.
#[derive(Debug, Clone)]
pub struct PoppetValve {
    pub id: u8,
    pub state: ValveState,
    /// Stroke position from 0 (fully seated/closed) to 1000 (fully open).
    pub stroke_mille: i64,
    /// Full stroke actuation time in milliseconds.
    pub stroke_time_ms: u64,
    /// Accumulated stroke count for seal wear prognostics.
    pub stroke_cycles: u64,
    /// Seat erosion metric (0 = pristine, 1000 = severe leakage).
    pub seat_wear: i64,
}

impl PoppetValve {
    pub fn new(id: u8) -> Self {
        Self {
            id,
            state: ValveState::Closed,
            stroke_mille: 0,
            stroke_time_ms: 35, // High-speed 35ms rapid closure
            stroke_cycles: 0,
            seat_wear: 0,
        }
    }

    /// Simulates valve mechanical motion over time step delta_ms.
    pub fn update(&mut self, target_open: bool, delta_ms: f64) {
        if self.state == ValveState::FaultStuck {
            return;
        }

        let rate = 1000.0 / (self.stroke_time_ms as f64);
        let step = (rate * delta_ms) as i64;

        if target_open {
            if self.stroke_mille < 1000 {
                self.state = ValveState::Opening;
                self.stroke_mille = (self.stroke_mille + step).min(1000);
                if self.stroke_mille == 1000 {
                    self.state = ValveState::Open;
                    self.stroke_cycles += 1;
                }
            }
        } else {
            if self.stroke_mille > 0 {
                self.state = ValveState::Closing;
                self.stroke_mille = (self.stroke_mille - step).max(0);
                if self.stroke_mille == 0 {
                    self.state = ValveState::Closed;
                }
            }
        }
    }
}

/// Hydraulic accumulator bank and manifold physical simulation.
#[derive(Debug, Clone)]
pub struct HydraulicHal {
    /// Upstream header pressure in bar (fixed point x10, e.g. 3450 = 345.0 bar).
    pub header_pressure: i64,
    /// Nitrogen accumulator bottle bank pressure in bar (fixed point x10).
    pub accumulator_pressure: i64,
    /// Triple-redundant pressure transmitters (A, B, C).
    pub sensor_a: i64,
    pub sensor_b: i64,
    pub sensor_c: i64,
    /// Primary HIPPS isolation poppet valves (dual-series 1oo2 fail-safe).
    pub valve_1: PoppetValve,
    pub valve_2: PoppetValve,
    /// Emergency trip coil status: 0 = energized (normal), 1 = de-energized (tripped).
    pub trip_coil_deenergized: bool,
    /// Alarm bitmask: bit 0: Hi-Hi trip, bit 1: TMR mismatch, bit 2: Acc low, bit 3: Watchdog.
    pub alarm_flags: i64,
    /// Monotonic system clock in milliseconds.
    pub time_ms: i64,
    /// Watchdog timer register.
    pub watchdog_counter: i64,
    /// Current Emergency Shutdown (ESD) level: 0 = normal, 1 = ESD-2, 2 = ESD-1, 3 = ESD-0.
    pub esd_level: i64,
    /// Volumetric flow rate through manifold in liters/min (fixed point x10).
    pub flow_rate: i64,
}

impl Default for HydraulicHal {
    fn default() -> Self {
        Self::new()
    }
}

impl HydraulicHal {
    pub fn new() -> Self {
        Self {
            header_pressure: 2500, // 250.0 bar nominal
            accumulator_pressure: 3100, // 310.0 bar pre-charge
            sensor_a: 2500,
            sensor_b: 2500,
            sensor_c: 2500,
            valve_1: PoppetValve::new(1),
            valve_2: PoppetValve::new(2),
            trip_coil_deenergized: false,
            alarm_flags: 0,
            time_ms: 0,
            watchdog_counter: 100,
            esd_level: 0,
            flow_rate: 1500,
        }
    }

    /// Advances the hydraulic physics model by dt_ms milliseconds.
    pub fn tick(&mut self, dt_ms: f64) {
        self.time_ms += dt_ms as i64;

        // Decrement watchdog
        self.watchdog_counter = self.watchdog_counter.saturating_sub(1);
        if self.watchdog_counter == 0 {
            self.alarm_flags |= 1 << 3; // Watchdog alarm bit
            self.trip_coil_deenergized = true;
        }

        // Valve target command: if trip coil is de-energized, fail-safe spring forces closed
        let target_open = !self.trip_coil_deenergized;
        self.valve_1.update(target_open, dt_ms);
        self.valve_2.update(target_open, dt_ms);

        // Compute effective flow area based on series valves
        let min_stroke = self.valve_1.stroke_mille.min(self.valve_2.stroke_mille);
        self.flow_rate = (1500 * min_stroke) / 1000;

        // Accumulator pressure consumption during actuation
        if self.valve_1.state == ValveState::Opening || self.valve_2.state == ValveState::Opening {
            self.accumulator_pressure = (self.accumulator_pressure - 2).max(1800);
        }
    }

    /// Injects an overpressure transient shock wave.
    pub fn inject_overpressure(&mut self, peak_bar_x10: i64) {
        self.header_pressure = peak_bar_x10;
        self.sensor_a = peak_bar_x10;
        self.sensor_b = peak_bar_x10;
        self.sensor_c = peak_bar_x10;
    }

    /// Injects a sensor drift or stuck fault into transducer channel A, B, or C.
    pub fn inject_sensor_fault(&mut self, channel: char, faulty_value: i64) {
        match channel {
            'A' | 'a' => self.sensor_a = faulty_value,
            'B' | 'b' => self.sensor_b = faulty_value,
            'C' | 'c' => self.sensor_c = faulty_value,
            _ => {}
        }
    }
}

impl MmioHandler for HydraulicHal {
    fn read(&mut self, x: i64, y: i64) -> Option<i64> {
        // Register map located at row Y = 100 in Funge-Space
        if y == 100 {
            match x {
                REG_TRANSDUCER_A => Some(self.sensor_a),
                REG_TRANSDUCER_B => Some(self.sensor_b),
                REG_TRANSDUCER_C => Some(self.sensor_c),
                REG_ACCUMULATOR_PRES => Some(self.accumulator_pressure),
                REG_VALVE_1_CMD => Some(self.valve_1.stroke_mille),
                REG_VALVE_2_CMD => Some(self.valve_2.stroke_mille),
                REG_TRIP_COIL => Some(if self.trip_coil_deenergized { 1 } else { 0 }),
                REG_ALARM_FLAGS => Some(self.alarm_flags),
                REG_MONOTONIC_TIME => Some(self.time_ms),
                REG_WATCHDOG_KICK => Some(self.watchdog_counter),
                REG_ESD_LEVEL => Some(self.esd_level),
                REG_FLOW_RATE => Some(self.flow_rate),
                _ => None,
            }
        } else {
            None
        }
    }

    fn write(&mut self, x: i64, y: i64, val: i64) -> bool {
        if y == 100 {
            match x {
                REG_TRIP_COIL => {
                    self.trip_coil_deenergized = val != 0;
                    true
                }
                REG_WATCHDOG_KICK => {
                    self.watchdog_counter = val.max(50);
                    // Clear watchdog alarm bit if refreshed
                    self.alarm_flags &= !(1 << 3);
                    true
                }
                REG_ESD_LEVEL => {
                    self.esd_level = val;
                    if val > 0 {
                        self.trip_coil_deenergized = true;
                    }
                    true
                }
                REG_ALARM_FLAGS => {
                    self.alarm_flags = val;
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poppet_valve_kinetics() {
        let mut valve = PoppetValve::new(1);
        assert_eq!(valve.state, ValveState::Closed);
        assert_eq!(valve.stroke_mille, 0);

        // Open valve over 35ms
        valve.update(true, 17.5);
        assert_eq!(valve.state, ValveState::Opening);
        assert!(valve.stroke_mille >= 450 && valve.stroke_mille <= 550);

        valve.update(true, 20.0);
        assert_eq!(valve.state, ValveState::Open);
        assert_eq!(valve.stroke_mille, 1000);
        assert_eq!(valve.stroke_cycles, 1);
    }

    #[test]
    fn test_hydraulic_hal_mmio_read_write() {
        let mut hal = HydraulicHal::new();
        hal.sensor_a = 3200;

        assert_eq!(hal.read(REG_TRANSDUCER_A, 100), Some(3200));
        assert_eq!(hal.read(REG_TRIP_COIL, 100), Some(0));

        // Command trip coil via MMIO write
        assert!(hal.write(REG_TRIP_COIL, 100, 1));
        assert!(hal.trip_coil_deenergized);
        assert_eq!(hal.read(REG_TRIP_COIL, 100), Some(1));
    }

    #[test]
    fn test_overpressure_injection_and_trip() {
        let mut hal = HydraulicHal::new();
        hal.valve_1.stroke_mille = 1000;
        hal.valve_2.stroke_mille = 1000;

        hal.inject_overpressure(4500); // 450.0 bar (well above 350.0 bar design trip)
        assert_eq!(hal.sensor_a, 4500);
        assert_eq!(hal.sensor_b, 4500);
        assert_eq!(hal.sensor_c, 4500);

        // Execute trip
        hal.trip_coil_deenergized = true;
        hal.tick(35.0); // Full stroke duration
        assert_eq!(hal.valve_1.state, ValveState::Closed);
        assert_eq!(hal.valve_2.state, ValveState::Closed);
        assert_eq!(hal.flow_rate, 0);
    }
}
