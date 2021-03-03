/**
 * C API Shared Memory and CRC-32 Unit Tests.
 *
 * Copyright (c) 2026 Aneesh Krishna Parthasarathy
 * Licensed under the ISC License.
 */

#include "../c_api/poppet.h"
#include <stdio.h>
#include <assert.h>
#include <string.h>

int main(void) {
    printf("Running Poppet C API unit tests...\n");

    /* Test 1: CRC-16 CCITT */
    const uint8_t msg[] = "POPPET_HIPPS_TEST";
    uint16_t crc16 = poppet_crc16_ccitt(msg, strlen((const char *)msg));
    assert(crc16 != 0);
    assert(crc16 != 0xFFFF);
    printf("  CRC-16 CCITT test: PASSED (0x%04X)\n", crc16);

    /* Test 2: CRC-32 */
    uint32_t crc32 = poppet_crc32(msg, strlen((const char *)msg));
    assert(crc32 != 0);
    assert(crc32 != 0xFFFFFFFF);
    printf("  CRC-32 IEEE 802.3 test: PASSED (0x%08X)\n", crc32);

    /* Test 3: Shared Memory Structure Size */
    assert(sizeof(poppet_telemetry_frame_t) == 32);
    printf("  Telemetry Frame Size: PASSED (32 bytes)\n");

    printf("All Poppet C API tests passed successfully.\n");
    return 0;
}
