# Poppet Register Map Specifications

All hardware registers are mapped to coordinate row Y = 100 in Funge-Space.
Reading via `g` retrieves the physical sensor value; writing via `p` commands hardware actuators.

## Funge-Space MMIO Registers (Row Y = 100)

| Register | Name | Type | Access | Description |
|:--------:|:-----|:----:|:------:|:------------|
| 0 | `REG_TRANSDUCER_A` | `i64` | R | Transmitter Channel A pressure (bar x 10) |
| 1 | `REG_TRANSDUCER_B` | `i64` | R | Transmitter Channel B pressure (bar x 10) |
| 2 | `REG_TRANSDUCER_C` | `i64` | R | Transmitter Channel C pressure (bar x 10) |
| 3 | `REG_ACCUMULATOR_PRES` | `i64` | R | Accumulator bank pressure (bar x 10) |
| 4 | `REG_VALVE_1_CMD` | `i64` | R/W | Poppet Valve 1 stroke (0 = seated, 1000 = open) |
| 5 | `REG_VALVE_2_CMD` | `i64` | R/W | Poppet Valve 2 stroke (0 = seated, 1000 = open) |
| 6 | `REG_TRIP_COIL` | `i64` | R/W | Solenoid trip coil (0 = energized safe, 1 = de-energized trip) |
| 7 | `REG_ALARM_FLAGS` | `i64` | R/W | System alarm bitmask (see bitfield definition below) |
| 8 | `REG_MONOTONIC_TIME` | `i64` | R | Monotonic system clock in milliseconds |
| 9 | `REG_WATCHDOG_KICK` | `i64` | R/W | Watchdog heartbeat countdown register |
| 10 | `REG_ESD_LEVEL` | `i64` | R/W | Active Emergency Shutdown level (0 = normal, 1..3 = ESD) |
| 11 | `REG_FLOW_RATE` | `i64` | R | Volumetric flow rate through manifold (liters/min x 10) |

## Alarm Bitfield Definition (`REG_ALARM_FLAGS`)

| Bit | Name | Severity | Description |
|:---:|:-----|:--------:|:------------|
| 0 | `ALARM_HI_HI_TRIP` | Critical | 2oo3 confirmed overpressure threshold exceeded |
| 1 | `ALARM_TMR_DEVIATION`| Warning | Single-channel sensor drift exceeds 15.0 bar tolerance |
| 2 | `ALARM_ACC_LOW_PRES` | Warning | Accumulator pressure below 210.0 bar minimum |
| 3 | `ALARM_WATCHDOG_EXP` | Critical | Safety kernel heartbeat timeout expired |
| 4 | `ALARM_VALVE_1_SLOW` | Warning | Poppet Valve 1 stroke travel time exceeded 45ms |
| 5 | `ALARM_VALVE_2_SLOW` | Warning | Poppet Valve 2 stroke travel time exceeded 45ms |
| 6 | `ALARM_SEAT_LEAKAGE` | Warning | Ultrasonic acoustic blow-by detected on closed seat |
| 7 | `ALARM_CAVITATION` | Warning | Fluid cavitation index below threshold |

## 32-Byte Binary Telemetry Frame Layout

Telemetry packets adhere to API Spec 16D subsea multiplex protocols:

```
Bytes 00-01 : Sync Word (0x5050 = 'PP')
Bytes 02-05 : Frame Sequence Number (uint32 big-endian)
Bytes 06-13 : Timestamp Milliseconds (int64 big-endian)
Bytes 14-15 : Voted Consensus Pressure (int16 big-endian, bar x 10)
Bytes 16-17 : Accumulator Bottle Pressure (int16 big-endian, bar x 10)
Bytes 18-19 : Poppet Valve 1 Stroke (int16 big-endian, 0..1000)
Bytes 20-21 : Poppet Valve 2 Stroke (int16 big-endian, 0..1000)
Bytes 22-23 : Manifold Flow Rate (int16 big-endian, L/min x 10)
Bytes 24-25 : System Alarm Bitmask (uint16 big-endian)
Byte  26    : Active ESD Level (uint8)
Byte  27    : Safety Trip Coil Status (uint8, 0 = normal, 1 = tripped)
Bytes 28-31 : IEEE 802.3 CRC-32 Checksum (uint32 big-endian)
```
