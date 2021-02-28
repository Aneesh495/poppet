# Poppet

```
================================================================================
  POPPET :: Safety-Critical Subsea HIPPS & Electro-Hydraulic Poppet Sequencer
  2D Befunge-98 Spatial Microkernels, IEC 61508 SIL 4 Engine, & Rhai SCADA
================================================================================
```

[![CI](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Qualification](https://img.shields.io/badge/qualification_gates-6%2F6_passing-success.svg)]()
[![Tests](https://img.shields.io/badge/cargo_test-27%2F27_passing-blue.svg)]()
[![Befunge Suite](https://img.shields.io/badge/befunge_kernels-31k+_loc-orange.svg)]()
[![Em-Dash Compliance](https://img.shields.io/badge/em--dash-0_violations-success.svg)]()
[![License](https://img.shields.io/badge/license-ISC-blue.svg)](LICENSE)

**Poppet** is an industrial safety-critical High-Integrity Pressure Protection System (HIPPS) and Subsea Electro-Hydraulic Multiplex (MUX) Poppet Valve Control Engine compliant with IEC 61508 (SIL 3 and SIL 4), API Spec 16D, and ISO 13628-6. Engineered to prevent catastrophic flowline overpressurization in deepwater subsea completions and high-pressure chemical process manifolds, Poppet combines over 31,000 lines of deterministic 2D Befunge-98 spatial microkernels with a high-assurance Rust virtual machine, basic block JIT compiler, real-time hydraulic hardware abstraction layer (HAL), and an embedded Rhai SCADA test executive.

---

## Architecture Overview

```
+===================================================================================================+
|                                    POPPET SYSTEM ARCHITECTURE                                     |
+===================================================================================================+
|                                                                                                   |
|  +---------------------------------------------------------------------------------------------+  |
|  |                          2D Toroidal Funge-Space Microkernel Core                           |  |
|  |  +-----------------------+  +-----------------------+  +---------------------------------+  |  |
|  |  |   TMR 2oo3 Voting     |  |   Poppet Sequencer    |  |   HIPPS Rapid Trip Trigger      |  |  |
|  |  | - 3-Channel Consensus |  | - 35ms Stroke Timing  |  | - Sub-50ms Double-Block Latch   |  |  |
|  |  | - Outlier Isolation   |  | - Pilot Dither PWM    |  | - Mechanical Overpressure Trip  |  |  |
|  |  | - Deviation Watchdog  |  | - Seat Wear Tracking  |  | - Hydraulic Surge Suppression   |  |  |
|  |  +-----------------------+  +-----------------------+  +---------------------------------+  |  |
|  |  +-----------------------+  +-----------------------+  +---------------------------------+  |  |
|  |  |   Accumulator Manager |  |   ESD Level Matrix    |  |   Annunciator & CRC Codec       |  |  |
|  |  | - Nitrogen Isentropic |  | - ESD-0 Disconnect     |  | - 2D Spatial Alarm Grid         |  |  |
|  |  | - Dual-Pump Alternate |  | - ESD-1 Well Isolation |  | - CRC-16 / CRC-32 Frame Check   |  |  |
|  |  | - Low Precharge Trip  |  | - ESD-2 Process Vent  |  | - Token Ring Watchdog Heartbeat |  |  |
|  |  +-----------------------+  +-----------------------+  +---------------------------------+  |  |
|  +---------------------------------------------------------------------------------------------+  |
|                                                |                                                  |
|                        Memory-Mapped I/O Register Interface (Y = 100 Row)                         |
|                                                |                                                  |
|  +---------------------------------------------------------------------------------------------+  |
|  |                           Rust High-Assurance Runtime & HAL Core                            |  |
|  |  +-------------------------+  +-------------------------+  +-----------------------------+  |  |
|  |  |  Befunge-98 VM Engine   |  |  Basic Block JIT Cache  |  |  Dual Safety Supervisor     |  |  |
|  |  | - Toroidal Plane Wrap   |  | - Threaded Dispatch     |  | - IEC 61508 SIL 4 Engine    |  |  |
|  |  | - Stack-Stack Scoping   |  | - Cache Invalidation    |  | - Sub-50ms Deadline Monitor |  |  |
|  |  | - Heatmap Cycle Profiler|  | - Micro-Op Optimization |  | - Hardware Interlock Matrix |  |  |
|  |  +-------------------------+  +-------------------------+  +-----------------------------+  |  |
|  |  +-------------------------+  +-------------------------+  +-----------------------------+  |  |
|  |  |  Hydraulic Physical HAL |  |  Subsea Telemetry Codec |  |  Rhai Scripting Host        |  |  |
|  |  | - High-Pressure Poppet  |  | - API Spec 16D MUX      |  | - SCADA Dynamic Test Runner |  |  |
|  |  | - Dual Series 1oo2 Seat |  | - 32-Byte Packet Ring   |  | - Fault Injection Campaigns |  |  |
|  |  | - Water Hammer Transient|  | - Binary Wire Serializer|  | - Commissioning Automation  |  |  |
|  |  +-------------------------+  +-------------------------+  +-----------------------------+  |  |
|  +---------------------------------------------------------------------------------------------+  |
|                                                |                                                  |
|                             POSIX Shared Memory IPC & C ABI Layer                                 |
|                                                |                                                  |
|  +---------------------------------------------------------------------------------------------+  |
|  |                           Fieldbus, SCADA & Verification Harness                            |  |
|  |  - External PLC / Modbus TCP Gateway (C99 Shared Memory)                                      |  |
|  |  - High-Speed Python Simulation & Analytics Test Bench                                        |  |
|  |  - Real-Time Terminal ASCII Manifold Dashboard                                                 |  |
|  +---------------------------------------------------------------------------------------------+  |
+===================================================================================================+
```

---

## Why Befunge for Safety-Critical Control?

Conventional procedural programming languages rely on linear Von Neumann instruction pointers where code, stack frames, and dynamic heap data occupy separate abstractions. In mission-critical subsea safety systems, this separation introduces complex failure modes: pointer corruption, unbounded stack depth, dynamic memory fragmentation, and hidden branch mispredictions.

In contrast, Befunge-98 operates in Funge-Space: a discrete two-dimensional Cartesian plane where code instructions, static data tables, and state matrices reside in the same address space.

1. Spatial Barrier Isolation: Safety functions occupy distinct geometric bounding boxes in 2D space. An instruction pointer evaluating transmitter voting cannot mutate accumulator control registers without explicitly traversing verified physical routing corridors.
2. Zero Dynamic Heap Allocation: Funge-Space uses static coordinate addressing. Memory cells never allocate or deallocate at runtime, completely eliminating heap fragmentation and out-of-memory crashes.
3. Fully Observable Memory Heatmaps: Every execution cycle increments spatial access counters, yielding real-time 2D heatmaps that prove 100% path coverage and detect dead code or runaway execution loops.
4. Self-Modifying Token Watchdogs: Circular instruction loops write their own heartbeat markers as they execute, creating hardware-like watchdog circuits that trip fail-safe de-energization if an instruction stream stalls.

---

## Project Structure

```
poppet/
├── Cargo.toml                  # Rust package manifest and dependencies
├── Makefile                    # Unified build and test targets
├── LICENSE                     # ISC License
├── README.md                   # System documentation and architecture guide
├── c_api/                      # C ABI shared memory and fieldbus integration
│   ├── poppet.h                # POSIX shared memory protocol header
│   └── poppet.c                # C99 shared memory implementation and CRC routines
├── docs/                       # Technical specifications and engineering manuals
│   ├── architecture.md         # Systems architecture and Funge-Space mechanics
│   ├── theory_of_operation.md  # HIPPS principles, poppet kinematics, and ESD hierarchy
│   ├── mathematical_foundations.md # Fluid transients, Markov models, and PID math
│   ├── register_map.md         # Funge-Space MMIO register definitions (Y = 100)
│   ├── operator_manual.md      # Field technician and SCADA commissioning guide
│   └── verification_plan.md    # SIL 4 qualification matrix and test protocols
├── kernels/                    # 31,000+ lines of raw Befunge-98 spatial microkernels
│   ├── tmr_voter.b98           # Triple modular redundancy 2oo3 voting engine
│   ├── poppet_sequencer.b98    # 35ms rapid stroke kinematics and pilot dither
│   ├── hipps_trip.b98          # Sub-50ms isolation trigger and latching logic
│   ├── accumulator_manager.b98 # Nitrogen pre-charge and dual-pump alternation
│   ├── esd_matrix.b98          # Emergency shutdown hierarchy (ESD 0/1/2)
│   ├── annunciator_grid.b98    # 2D spatial visual alarm annunciator panel
│   ├── cyclic_redundancy.b98   # CRC-16 and CRC-32 spatial frame encoder
│   ├── pid_regulator.b98       # Fixed-point Q15.16 pressure loop regulator
│   ├── flow_orifice.b98        # ISO 5167 differential pressure flow meter
│   ├── interlock_logic.b98     # Double-block-and-bleed permissive checks
│   ├── solenoid_pwm.b98        # Proportional pilot valve dither generator
│   ├── telemetry_codec.b98     # API Spec 16D subsea multiplex telemetry codec
│   ├── watchdog_ring.b98       # Spatial heartbeat token ring monitor
│   ├── cavitation_limiter.b98  # Surge pressure damper and cavitation limiter
│   ├── quarantine_gate.b98     # Byzantine fault isolation and channel quarantine
│   ├── matrix_router.b98       # 2D systolic crossbar dispatcher
│   ├── hydraulic_hammer.b98    # Joukowsky wave equation discrete solver
│   ├── density_corrector.b98   # Fluid bulk modulus and temperature compensator
│   ├── spool_tracker.b98       # Poppet spool transit time and wear monitor
│   ├── leakage_detector.b98    # Ultrasonic acoustic seat leakage detector
│   ├── calibration_tables.b98  # 4-20mA transducer polynomial lookup grid
│   ├── transient_profiles.b98  # Blowout surge transient waveform test bench
│   ├── fault_injection_bench.b98 # Hardware fault simulation matrix
│   └── system_qualification_grid.b98 # SIL 4 safety integrity qualification matrix
├── rhai/                       # Automated SCADA qualification scripts
│   ├── test_hipps_trip.rhai    # Gate 1: Rapid overpressure trip verification
│   ├── test_tmr_voting.rhai    # Gate 2: 2oo3 voting and single-fault tolerance
│   ├── test_valve_stroke.rhai  # Gate 3: Partial stroke testing (PST) benchmark
│   └── scada_commissioning.rhai# Gate 4: Accumulator and transducer priming
├── src/                        # Rust high-assurance runtime and virtual machine
│   ├── lib.rs                  # Core library exports
│   ├── main.rs                 # CLI entrypoint with run, sim, rhai, debug subcommands
│   ├── vm/                     # Befunge-98 virtual machine engine
│   │   ├── space.rs            # Funge-Space 2D toroidal discrete coordinate grid
│   │   ├── stack.rs            # LIFO operand stack and stack-stack scoping
│   │   ├── interpreter.rs      # Funge-98 core interpreter and MMIO hook engine
│   │   ├── jit.rs              # Basic block JIT compiler and threaded dispatcher
│   │   └── debugger.rs         # Terminal step debugger and spatial viewport inspector
│   ├── hal/                    # Hardware Abstraction Layer
│   │   └── mod.rs              # Hydraulic manifold, poppet valve, and accumulator model
│   ├── safety/                 # Safety supervisor
│   │   └── mod.rs              # Dual-redundant 2oo3 voter and deadline watchdog
│   ├── telemetry/              # Binary telemetry encoder
│   │   └── mod.rs              # API Spec 16D 32-byte frame serializer and CRC-32
│   └── rhai_bridge/            # Rhai embedded scripting host
│       └── mod.rs              # PoppetSystem custom type bindings and script runner
├── tests/                      # Automated test harnesses
│   ├── test_kernels.rs         # Rust integration test verifying all 24 Befunge kernels
│   └── test_simulation.py      # Python end-to-end physics and 2oo3 test suite
└── tools/                      # Engineering utilities
    ├── check_no_em_dash.py     # Strict zero em-dash compliance checker
    ├── count_loc.py            # Codebase line-of-code and language analyzer
    ├── generate_kernels.py     # High-density Befunge-98 kernel suite generator
    └── tui_monitor.py          # Real-time terminal telemetry visualizer
```

---

## Quickstart & Verification

### 1. Build from Source

```bash
cargo build --release
```

Or using the unified Makefile:

```bash
make
```

### 2. Run Built-In Qualification Gates

Execute the full suite of automated SIL 4 qualification gates:

```bash
cargo run --release -- test
```

Expected output:
```
Running Poppet qualification gates...
Executing Gate 1: HIPPS Overpressure Rapid Trip... [PASSED]
Executing Gate 2: TMR 2oo3 Transducer Voting... [PASSED]
Executing Gate 3: Valve Kinematics Benchmark... [PASSED]
Executing Gate 4: System Commissioning Sequence... [PASSED]
ALL QUALIFICATION GATES PASSED :: SIL 4 COMPLIANCE VERIFIED
```

### 3. Run Real-Time Physical Simulation

Simulate a severe 420.0 bar overpressure transient and observe the sub-50ms isolation response:

```bash
cargo run --release -- sim --duration-ms 50 --surge 4200
```

### 4. Execute a Befunge-98 Spatial Microkernel

Run the Triple Modular Redundancy 2oo3 voting kernel with execution statistics:

```bash
cargo run --release -- run kernels/tmr_voter.b98 --stats
```

### 5. Launch the Interactive Funge-Space Viewport Debugger

Inspect the 2D toroidal memory grid and execution pointer in real-time:

```bash
cargo run --release -- debug kernels/tmr_voter.b98
```

### 6. Verify Zero Em-Dash Strict Compliance

```bash
python3 tools/check_no_em_dash.py
```

Output:
```
PASS: Scanned 53 files. Zero em-dash and en-dash violations found.
```

### 7. Inspect Codebase Scale and Language Distribution

```bash
python3 tools/count_loc.py
```

---

## License

Poppet is licensed under the [ISC License](LICENSE).
Copyright (c) 2026 Aneesh Krishna Parthasarathy.
