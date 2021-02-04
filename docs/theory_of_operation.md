# Poppet Theory of Operation

## High-Integrity Pressure Protection System (HIPPS) Principles

A High-Integrity Pressure Protection System (HIPPS) is a safety-instrumented system (SIS) engineered to act as a definitive physical barrier against pipeline or pressure vessel rupture caused by overpressure events.

In conventional subsea oil and gas developments or chemical process plants, facilities are designed to withstand full wellhead shut-in pressure (e.g. 10,000 to 15,000 PSI). However, full-rating pipelines require massive wall thicknesses, heavy steel tonnage, and exorbitant capital expenditure. HIPPS allows downstream flowlines and processing equipment to be designed to lower pressure ratings by guaranteeing an ultra-reliable, SIL 4 sub-50ms isolation barrier before overpressure shock waves can reach downstream piping.

### Safety Instrumented Function (SIF) Requirements

Poppet implements the complete HIPPS SIF loop:
1. Sensing Element: Three independent 4-20mA pressure transmitters arranged in a 2-out-of-3 (2oo3) voting topology.
2. Logic Solver: Spatial Befunge-98 microkernel matrix executing cycle-accurate voting and cause-and-effect interlocks.
3. Final Element: Dual fast-acting hydraulic cartridge poppet valves in a 1-out-of-2 (1oo2) fail-safe series arrangement.

```
+-------------------------------------------------------------------------------+
|                            HIPPS SIF FUNCTIONAL LOOP                          |
+-------------------------------------------------------------------------------+
|                                                                               |
|  [Transducer A] ---+                                                          |
|                    |                                                          |
|  [Transducer B] ---+--> [2oo3 Voting Logic] --> [Trip Coil] --> [Poppet 1]    |
|                    |    (Befunge Kernel)       (24V De-energize)     |        |
|  [Transducer C] ---+                                            [Poppet 2]    |
|                                                                               |
+-------------------------------------------------------------------------------+
```

## Bi-Stable Hydraulic Poppet Valve Kinematics

Standard gate and ball valves require hundreds of milliseconds or several seconds to achieve mechanical isolation due to high stem friction and massive actuator displacement. In extreme subsea pressure surges, closing times above 100 milliseconds risk catastrophic flowline overpressurization.

Poppet utilizes bi-stable balanced cartridge poppet valves:
- Ultra-Fast Stroke Time: 35 milliseconds from full open (1000 mille) to metal-to-metal seating (0 mille).
- Mechanical Spring Assist: Hydraulic pilot pressure maintains the valve open against heavy internal mechanical spring packs. When the 24V DC pilot solenoid coil de-energizes, the pilot fluid rapidly dumps into the subsea return reservoir, and the mechanical spring slams the poppet shut with zero external electrical power required (fail-safe closed).
- Dual-Series Redundancy: Two identical poppet valves are mounted in series. Complete flow arrest occurs if either valve seats successfully (1oo2 voting architecture for final elements).

## 2-out-of-3 (2oo3) Sensor Voting and Fault Isolation

The sensing architecture must balance two competing reliability imperatives:
1. Safety Availability (PFDavg): The system must never fail to trip when genuine overpressure occurs.
2. Spurious Trip Resistance (MTTFspurious): A single failing transmitter must not cause an unscheduled, costly emergency shutdown.

In 2oo3 voting:
- A trip command is issued if and only if at least two of the three transmitter channels exceed the high-high trip setpoint (345.0 bar / 5000 PSI).
- If Channel A drifts high (e.g. 450.0 bar) due to an open circuit or transducer failure while Channels B and C read normal operating pressure (250.0 bar), the median output remains 250.0 bar. The system flags Channel A as degraded, sets alarm bit 1, and continues normal production without disruption.
- If genuine overpressure strikes Channels A and B, both exceed setpoint, forcing the median above 345.0 bar and tripping the poppet valves in under 10 milliseconds.

## Emergency Shutdown (ESD) Hierarchy

Poppet manages a multi-tiered emergency shutdown state machine:
- Normal (Level 0): All poppet valves energized open, continuous flow, watchdog refreshed every 100 cycles.
- ESD Level 2 (Process Header Block): Non-critical upstream process upset. Throttles poppet valves to 50% partial stroke and sounds visual alarms on the annunciator grid.
- ESD Level 1 (Wellhead Flowline Isolation): Confirmed 2oo3 overpressure trip. Rapidly de-energizes pilot coils, closing both series poppet valves in 35ms and locking out manual reset until pressure drops below permissive threshold (325.0 bar).
- ESD Level 0 (Total Subsea Disconnect): Catastrophic riser or structural rupture command. Executes full manifold depressurization, activates accumulator high-flow dump circuits, and isolates hydraulic power units.
