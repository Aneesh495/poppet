/**
 * Poppet Subsea Electro-Hydraulic C API Implementation.
 *
 * Implements POSIX shared memory, CRC algorithms, and fieldbus helpers.
 *
 * Copyright (c) 2026 Aneesh Krishna Parthasarathy
 * Licensed under the ISC License.
 */

#include "poppet.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/mman.h>
#include <sys/stat.h>

int poppet_shm_create_or_open(poppet_shm_layout_t **out_ptr, int create) {
    if (!out_ptr) return -1;

    int flags = create ? (O_CREAT | O_RDWR) : O_RDWR;
    mode_t mode = 0666;
    int fd = shm_open(POPPET_SHM_NAME, flags, mode);
    if (fd < 0) {
        return -1;
    }

    if (create) {
        if (ftruncate(fd, sizeof(poppet_shm_layout_t)) != 0) {
            close(fd);
            return -2;
        }
    }

    void *addr = mmap(NULL, sizeof(poppet_shm_layout_t), PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    close(fd);

    if (addr == MAP_FAILED) {
        return -3;
    }

    poppet_shm_layout_t *shm = (poppet_shm_layout_t *)addr;
    if (create) {
        memset(shm, 0, sizeof(poppet_shm_layout_t));
        shm->magic = POPPET_MAGIC;
        shm->version = 1;
    }

    *out_ptr = shm;
    return 0;
}

void poppet_shm_detach(poppet_shm_layout_t *ptr) {
    if (ptr) {
        munmap(ptr, sizeof(poppet_shm_layout_t));
    }
}

int poppet_shm_unlink(void) {
    return shm_unlink(POPPET_SHM_NAME);
}

uint16_t poppet_crc16_ccitt(const uint8_t *buffer, size_t length) {
    uint16_t crc = 0xFFFF;
    for (size_t i = 0; i < length; ++i) {
        crc ^= (uint16_t)buffer[i] << 8;
        for (int b = 0; b < 8; ++b) {
            if (crc & 0x8000) {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    return crc;
}

uint32_t poppet_crc32(const uint8_t *buffer, size_t length) {
    uint32_t crc = 0xFFFFFFFF;
    for (size_t i = 0; i < length; ++i) {
        crc ^= buffer[i];
        for (int b = 0; b < 8; ++b) {
            uint32_t mask = -(crc & 1);
            crc = (crc >> 1) ^ (0xEDB88320 & mask);
        }
    }
    return ~crc;
}

int poppet_command_emergency_trip(poppet_shm_layout_t *shm, uint8_t esd_level) {
    if (!shm) return -1;
    shm->command_trip_coil = 1;
    shm->command_esd_level = esd_level;
    return 0;
}

int poppet_refresh_watchdog(poppet_shm_layout_t *shm) {
    if (!shm) return -1;
    shm->watchdog_heartbeat++;
    return 0;
}
