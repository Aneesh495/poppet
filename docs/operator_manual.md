# Poppet Operator Manual & Commissioning Guide

This manual serves field engineers, instrument technicians, and SCADA console operators responsible for the commissioning, proof testing, and operation of the Poppet High-Integrity Pressure Protection System.

## Normal Operating States

During standard production, the console display should indicate:
- Safety State: `NORMAL [ENERGIZED]`
- Trip Coil: `0 (24V DC energized)`
- ESD Level: `0 (Normal)`
- Valve 1 and 2 Strokes: `1000 mille (100% full open)`
- Header Pressure: `250.0 bar nominal (operating envelope: 200.0 to 300.0 bar)`
- Accumulator Pressure: `310.0 bar nominal (minimum limit: 210.0 bar)`

## Partial Stroke Testing (PST) Procedure

To prevent mechanical stiction and confirm solenoid pilot valve availability without disrupting active flowline production, operators must perform automated Partial Stroke Testing (PST) every 30 days:

1. Initiate the automated PST sequence via Rhai CLI:
   `poppet rhai rhai/test_valve_stroke.rhai`
2. The controller pulses the pilot solenoid, displacing Poppet Valve 1 from 1000 mille to 850 mille (15% stroke travel).
3. The displacement transducer verifies stroke initiation within 12 milliseconds.
4. Pilot pressure is immediately restored, returning the valve to 1000 mille.
5. The procedure is repeated for Poppet Valve 2.
6. The test logs travel velocity, seal friction margin, and stroke duration into the STDF record.

## Emergency Trip Response & Reset Protocols

When an overpressure transient exceeding 345.0 bar occurs:
1. 2oo3 Voting trips the solenoid coil in sub-10ms time.
2. Poppet Valves 1 and 2 slam closed via mechanical springs in 35ms.
3. The alarm annunciator latches `ALARM_HI_HI_TRIP` (Bit 0).
4. The system transitions into a locked fail-safe state.

### Reset Prerequisites

The system enforces strict software and mechanical interlocks preventing premature resets:
- Transducer readings on all three channels must remain below 325.0 bar for at least 60 consecutive seconds.
- Accumulator bank pressure must exceed 280.0 bar.
- The operator must acknowledge the latching alarm on the annunciator grid.
- Reset command is issued via SCADA or CLI:
  `poppet run kernels/interlock_logic.b98`
