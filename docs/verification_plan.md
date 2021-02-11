# Poppet Verification & Validation Plan

## Safety Integrity Level (SIL 4) Qualification Matrix

The Poppet verification suite validates functional safety compliance against IEC 61508 Parts 1 to 7 and API Spec 16D.

| Gate | Title | Objective | Target Metric | Status |
|:----:|:------|:----------|:-------------:|:------:|
| 1 | HIPPS Overpressure Rapid Trip | Sub-50ms isolation upon 420.0 bar surge | Latency < 45ms | PASSED |
| 2 | TMR 2oo3 Transducer Voting | Immunity to single-channel 450.0 bar stuck fault | 0 Spurious Trips | PASSED |
| 3 | Valve Kinematics Benchmark | 35ms mechanical stroke profile and PST | Stroke Time <= 35ms | PASSED |
| 4 | System Commissioning Sequence | Accumulator pre-charge and 4-20mA calibration | Continuity 100% | PASSED |
| 5 | Befunge Microkernel Verification | All 24 Befunge spatial kernels parse and run | 24/24 Verified | PASSED |
| 6 | Zero Em-Dash Strict Compliance | Scans all code, docs, and comments for dashes | 0 Violations | PASSED |

## Automated Fault Injection Methodology

The verification harness injects anomalous physical disturbances into the simulation:
1. Stuck-at-High Fault: Transducer A holds 450.0 bar while Channels B and C experience normal pressure. Verified: system does NOT trip.
2. Stuck-at-Low Fault: Transducer B holds 0.0 bar while Channels A and C experience genuine 360.0 bar overpressure. Verified: system trips on Channels A and C consensus.
3. Transducer Drift Fault: Transducer C drifts by 2.0 bar/sec until exceeding the 15.0 bar deviation threshold. Verified: system isolates Channel C and raises diagnostic alarm bit 1.
4. Watchdog Starvation: Simulated execution freeze by withholding `REG_WATCHDOG_KICK`. Verified: hardware watchdog times out at 100ms and triggers fail-safe emergency de-energization.
