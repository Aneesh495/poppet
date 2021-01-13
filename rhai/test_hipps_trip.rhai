// Poppet SCADA Automated Qualification: HIPPS Rapid Overpressure Trip
//
// Injects a rapid pressure surge (420.0 bar) into the hydraulic manifold,
// verifies sub-50ms isolation trigger, and confirms fail-safe poppet seating.
//
// Copyright (c) 2026 Aneesh Krishna Parthasarathy
// Licensed under the ISC License.

fn run_hipps_trip_test(sys) {
    print("--- POPPET SCADA QUALIFICATION GATE 1: HIPPS OVERPRESSURE TRIP ---");
    
    // 1. Verify baseline pressure
    let p_initial = sys.get_pressure_a();
    print("Baseline pressure Channel A: " + p_initial + " (bar x 10)");
    if sys.is_tripped() {
        throw "System unexpectedly started in tripped state";
    }

    // 2. Inject rapid overpressure shock wave (420.0 bar)
    print("Injecting severe overpressure transient surge: 420.0 bar...");
    sys.inject_surge(4200);

    // 3. Tick physics for 10ms
    sys.tick_physics(10.0);

    // 4. Verify trip coil de-energized
    if !sys.is_tripped() {
        throw "CRITICAL FAILURE: Safety trip coil failed to de-energize within 10ms deadline!";
    }
    print("PASS: Trip coil successfully de-energized within 10ms deadline.");

    // 5. Advance physics through full 35ms mechanical stroke
    sys.tick_physics(35.0);

    // 6. Confirm dual series valves fully closed
    let stroke1 = sys.get_valve_1_stroke();
    let stroke2 = sys.get_valve_2_stroke();
    let flow = sys.get_flow_rate();

    print("Valve 1 stroke: " + stroke1 + " / 1000");
    print("Valve 2 stroke: " + stroke2 + " / 1000");
    print("Residual flow rate: " + flow + " L/min");

    if stroke1 != 0 || stroke2 != 0 {
        throw "CRITICAL FAILURE: Poppet valves did not achieve zero-stroke mechanical closure!";
    }

    if flow != 0 {
        throw "CRITICAL FAILURE: Manifold flow not fully arrested!";
    }

    print("PASS: Dual-series poppet valves seated at 0 stroke. Zero flow confirmed.");
    print("--- QUALIFICATION GATE 1 COMPLETED SUCCESSFULLY ---");
    return true;
}

run_hipps_trip_test(sys);
