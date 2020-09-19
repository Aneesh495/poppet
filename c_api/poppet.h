/**
 * Poppet Subsea Electro-Hydraulic C API and Shared Memory Protocol.
 *
 * Defines POSIX shared memory interfaces, telemetry frame structures,
 * and high-integrity trip commands for external SCADA / Modbus integration.
 *
 * Copyright (c) 2026 Aneesh Krishna Parthasarathy
 * Licensed under the ISC License.
 */

#ifndef POPPET_H
#define POPPET_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

#define POPPET_SHM_NAME "/poppet_telemetry_shm"
#define POPPET_MAGIC 0x50504554 /* 'PPET' */

#pragma pack(push, 1)

/**
 * Real-time 32-byte binary telemetry frame layout.
 */
typedef struct {
    uint16_t sync_word;        /* 0x5050 */
    uint32_t frame_seq;        /* Monotonic sequence number */
    int64_t  timestamp_ms;     /* Epoch timestamp in ms */
    int16_t  pressure_voted;   /* Voted header pressure (bar x 10) */
    int16_t  accumulator_bar;  /* Accumulator pre-charge (bar x 10) */
    int16_t  valve_1_stroke;   /* Valve 1 stroke (0..1000) */
    int16_t  valve_2_stroke;   /* Valve 2 stroke (0..1000) */
    int16_t  flow_lpm;         /* Flow rate in liters/min (x 10) */
    uint16_t alarm_flags;      /* System alarm bitmask */
    uint8_t  esd_level;        /* Active ESD level (0..3) */
    uint8_t  trip_status;      /* 1 = tripped, 0 = healthy */
    uint32_t crc32;            /* IEEE 802.3 CRC-32 checksum */
} poppet_telemetry_frame_t;

/**
 * Shared memory IPC region for bidirectional host exchange.
 */
typedef struct {
    uint32_t magic;
    uint32_t version;
    uint64_t heartbeat_tick;
    poppet_telemetry_frame_t telemetry;
    int32_t command_trip_coil;
    int32_t command_esd_level;
    uint32_t watchdog_heartbeat;
} poppet_shm_layout_t;

#pragma pack(pop)

/* Lifecycle and IPC prototypes */
int poppet_shm_create_or_open(poppet_shm_layout_t **out_ptr, int create);
void poppet_shm_detach(poppet_shm_layout_t *ptr);
int poppet_shm_unlink(void);

/* Checksum calculation utilities */
uint16_t poppet_crc16_ccitt(const uint8_t *buffer, size_t length);
uint32_t poppet_crc32(const uint8_t *buffer, size_t length);

/* Safety Command Functions */
int poppet_command_emergency_trip(poppet_shm_layout_t *shm, uint8_t esd_level);
int poppet_refresh_watchdog(poppet_shm_layout_t *shm);

#ifdef __cplusplus
}
#endif

#endif /* POPPET_H */
