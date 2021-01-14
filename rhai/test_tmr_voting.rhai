// Poppet SCADA Automated Qualification: 2oo3 Voting & Fault Tolerance
//
// Injects single-channel transmitter faults (stuck-at-high 450.0 bar) to verify
// spurious trip immunity, followed by dual-channel genuine overpressure.
//
// Copyright (c) 2026 Aneesh Krishna Parthasarathy
// Licensed under the ISC License.

fn run_tmr_voting_test(sys) {
    print("--- POPPET SCADA QUALIFICATION GATE 2: TMR 2oo3 VOTING LOGIC ---");

    // Phase 1: Single-channel fault injection on Channel A (450.0 bar)
    // Channels B and C remain healthy at nominal 250.0 bar
    print("Injecting single-channel transducer failure: Channel A stuck-at 450.0 bar...");
    sys.set_pressure_a(4500);
    sys.set_pressure_b(2500);
    sys.set_pressure_c(2510);
    sys.tick_physics(10.0);

    // Verify system did NOT trip on single faulty channel
    if sys.is_tripped() {
        throw "CRITICAL FAILURE: Single faulty channel caused spurious trip! 2oo3 voting violated.";
    }
    print("PASS: Single-channel outlier rejected. System maintained operational flow.");

    // Phase 2: Dual-channel surge on Channels A and B (360.0 bar)
    print("Injecting dual-channel confirmed overpressure: Channels A and B at 360.0 bar...");
    sys.set_pressure_a(3600);
    sys.set_pressure_b(3620);
    sys.set_pressure_c(2500);
    sys.tick_physics(10.0);

    // Verify 2-of-3 trip demand
    if !sys.is_tripped() {
        throw "CRITICAL FAILURE: 2-of-3 confirmed overpressure failed to trip system!";
    }
    print("PASS: 2-of-3 confirmed overpressure correctly triggered emergency trip.");
    print("--- QUALIFICATION GATE 2 COMPLETED SUCCESSFULLY ---");
    return true;
}

run_tmr_voting_test(sys);
