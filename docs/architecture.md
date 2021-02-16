# Poppet System Architecture

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

## System Overview

Poppet is a safety-critical High-Integrity Pressure Protection System (HIPPS) and Subsea Electro-Hydraulic Multiplex (MUX) Poppet Valve Control Engine. The system is designed to satisfy the rigorous safety availability and reliability standards of IEC 61508 (SIL 3 and SIL 4), API Spec 16D, and ISO 13628-6 for deepwater subsea completions and chemical process plants.

The architecture decouples the execution into two complementary domains:
1. Spatial Safety Microkernels (Befunge-98): Over 30,000 lines of deterministic, cycle-accurate spatial kernels executing 2-out-of-3 (2oo3) sensor voting, rapid valve actuation timing, accumulator replenishment, and emergency shutdown sequencing within a 2D toroidal Funge-Space.
2. High-Assurance Host Runtime (Rust): A deterministic virtual machine, basic block JIT compiler, real-time hydraulic hardware abstraction layer (HAL), and dual-redundant supervisory watchdog.
3. SCADA Dynamic Scripting (Rhai): Embedded scripting interface for operator commissioning, automated proof testing, and fault-injection scenarios.
4. Host IPC Bridge (C99): POSIX shared memory interface for fieldbus coupling and remote SCADA monitoring.

## 2D Spatial Execution Architecture

Traditional linear Von Neumann instruction streams suffer from execution opacity and dynamic branch prediction complexity. In contrast, Befunge-98 operates in Funge-Space: a discrete two-dimensional Cartesian plane where code and memory cells are topologically co-located.

In Poppet, this spatial topology provides distinct engineering guarantees:
1. Spatial Boundary Isolation: Independent safety functions reside in isolated geometric bounding boxes. Upstream transmitter voting logic cannot write to accumulator control registers without physically routing an instruction pointer across verified boundary corridors.
2. Deterministic Bounded Execution: Funge-Space contains zero dynamic heap allocations. Memory cells are fixed coordinate locations, completely eliminating heap fragmentation and out-of-memory panics.
3. Inspectable State Heatmaps: Every instruction cycle records spatial memory access heatmaps, enabling 100% path coverage analysis and real-time visualization of executing code paths.

## Hardware Abstraction Layer & MMIO Mapping

Communication between the physical hydraulic manifold and the Befunge spatial kernels occurs through Memory-Mapped I/O (MMIO) registers located at row Y = 100 in Funge-Space:

- `REG_TRANSDUCER_A (0, 100)`: 4-20mA Transmitter Channel A (bar x 10)
- `REG_TRANSDUCER_B (1, 100)`: 4-20mA Transmitter Channel B (bar x 10)
- `REG_TRANSDUCER_C (2, 100)`: 4-20mA Transmitter Channel C (bar x 10)
- `REG_ACCUMULATOR_PRES (3, 100)`: Nitrogen Accumulator Bank Pressure (bar x 10)
- `REG_VALVE_1_CMD (4, 100)`: Poppet Valve 1 Stroke Position (0..1000 mille)
- `REG_VALVE_2_CMD (5, 100)`: Poppet Valve 2 Stroke Position (0..1000 mille)
- `REG_TRIP_COIL (6, 100)`: Solenoid Trip Coil Command (0 = Energized, 1 = Tripped)
- `REG_ALARM_FLAGS (7, 100)`: System Alarm Bitmask
- `REG_MONOTONIC_TIME (8, 100)`: Monotonic Clock Timestamp (milliseconds)
- `REG_WATCHDOG_KICK (9, 100)`: Watchdog Refresh Heartbeat Register
- `REG_ESD_LEVEL (10, 100)`: Active Emergency Shutdown Level (0..3)
- `REG_FLOW_RATE (11, 100)`: Volumetric Manifold Flow Rate (liters/min x 10)

Whenever a Befunge instruction pointer executes `g` (get) or `p` (put) on row 100, the VM intercepts the request and reads or writes physical hardware registers in sub-microsecond time.
