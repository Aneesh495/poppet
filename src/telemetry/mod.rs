//! Binary Telemetry Frame Serializer and Integrity Verification Engine.
//!
//! Encodes real-time subsea electro-hydraulic manifold telemetry compliant with
//! API Spec 16D multiplex framing protocols and computes CRC-16 / CRC-32 checksums.
//!
//! Copyright (c) 2026 Aneesh Krishna Parthasarathy
//! Licensed under the ISC License.

use byteorder::{BigEndian, ByteOrder};
use crc32fast::Hasher;
use crate::hal::HydraulicHal;

/// Protocol sync word identifying Poppet telemetry frames (0x5050 = 'PP').
pub const TELEM_SYNC_WORD: u16 = 0x5050;

/// Standard binary telemetry packet layout (32 bytes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryPacket {
    pub sync: u16,
    pub frame_seq: u32,
    pub timestamp_ms: i64,
    pub pressure_voted: i16,
    pub accumulator_bar: i16,
    pub valve_1_mille: i16,
    pub valve_2_mille: i16,
    pub flow_lpm: i16,
    pub alarm_flags: u16,
    pub esd_level: u8,
    pub trip_status: u8,
    pub crc32: u32,
}

impl TelemetryPacket {
    /// Serializes telemetry packet into 32-byte big-endian wire format.
    pub fn serialize(&self) -> [u8; 32] {
        let mut buf = [0u8; 32];
        BigEndian::write_u16(&mut buf[0..2], self.sync);
        BigEndian::write_u32(&mut buf[2..6], self.frame_seq);
        BigEndian::write_i64(&mut buf[6..14], self.timestamp_ms);
        BigEndian::write_i16(&mut buf[14..16], self.pressure_voted);
        BigEndian::write_i16(&mut buf[16..18], self.accumulator_bar);
        BigEndian::write_i16(&mut buf[18..20], self.valve_1_mille);
        BigEndian::write_i16(&mut buf[20..22], self.valve_2_mille);
        BigEndian::write_i16(&mut buf[22..24], self.flow_lpm);
        BigEndian::write_u16(&mut buf[24..26], self.alarm_flags);
        buf[26] = self.esd_level;
        buf[27] = self.trip_status;

        // Compute CRC32 over payload bytes [0..28]
        let mut hasher = Hasher::new();
        hasher.update(&buf[0..28]);
        let crc = hasher.finalize();

        BigEndian::write_u32(&mut buf[28..32], crc);
        buf
    }

    /// Deserializes 32-byte binary packet and verifies CRC32 integrity.
    pub fn deserialize(buf: &[u8]) -> Option<Self> {
        if buf.len() < 32 {
            return None;
        }

        let sync = BigEndian::read_u16(&buf[0..2]);
        if sync != TELEM_SYNC_WORD {
            return None;
        }

        let expected_crc = BigEndian::read_u32(&buf[28..32]);
        let mut hasher = Hasher::new();
        hasher.update(&buf[0..28]);
        if hasher.finalize() != expected_crc {
            return None; // CRC mismatch
        }

        Some(Self {
            sync,
            frame_seq: BigEndian::read_u32(&buf[2..6]),
            timestamp_ms: BigEndian::read_i64(&buf[6..14]),
            pressure_voted: BigEndian::read_i16(&buf[14..16]),
            accumulator_bar: BigEndian::read_i16(&buf[16..18]),
            valve_1_mille: BigEndian::read_i16(&buf[18..20]),
            valve_2_mille: BigEndian::read_i16(&buf[20..22]),
            flow_lpm: BigEndian::read_i16(&buf[22..24]),
            alarm_flags: BigEndian::read_u16(&buf[24..26]),
            esd_level: buf[26],
            trip_status: buf[27],
            crc32: expected_crc,
        })
    }

    /// Captures current state from HydraulicHal and builds a telemetry packet.
    pub fn capture(hal: &HydraulicHal, frame_seq: u32, voted_pressure: i64) -> Self {
        let mut pkt = Self {
            sync: TELEM_SYNC_WORD,
            frame_seq,
            timestamp_ms: hal.time_ms,
            pressure_voted: (voted_pressure / 10) as i16,
            accumulator_bar: (hal.accumulator_pressure / 10) as i16,
            valve_1_mille: hal.valve_1.stroke_mille as i16,
            valve_2_mille: hal.valve_2.stroke_mille as i16,
            flow_lpm: (hal.flow_rate / 10) as i16,
            alarm_flags: hal.alarm_flags as u16,
            esd_level: hal.esd_level as u8,
            trip_status: if hal.trip_coil_deenergized { 1 } else { 0 },
            crc32: 0,
        };
        let bytes = pkt.serialize();
        pkt.crc32 = BigEndian::read_u32(&bytes[28..32]);
        pkt
    }
}

/// Computes CRC-16-CCITT checksum over a byte slice.
pub fn compute_crc16_ccitt(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_packet_serialization_roundtrip() {
        let hal = HydraulicHal::new();
        let pkt = TelemetryPacket::capture(&hal, 1042, 2500);

        let bytes = pkt.serialize();
        assert_eq!(bytes.len(), 32);

        let decoded = TelemetryPacket::deserialize(&bytes);
        assert!(decoded.is_some());
        let unp = decoded.unwrap();
        assert_eq!(unp.frame_seq, 1042);
        assert_eq!(unp.pressure_voted, 250);
        assert_eq!(unp.sync, TELEM_SYNC_WORD);
    }

    #[test]
    fn test_telemetry_crc_corrupt_detection() {
        let hal = HydraulicHal::new();
        let pkt = TelemetryPacket::capture(&hal, 1, 2500);
        let mut bytes = pkt.serialize();

        // Corrupt byte at index 15
        bytes[15] ^= 0xFF;
        assert_eq!(TelemetryPacket::deserialize(&bytes), None);
    }

    #[test]
    fn test_crc16_ccitt() {
        let sample = b"POPPET-16D-TELEM";
        let crc = compute_crc16_ccitt(sample);
        assert_ne!(crc, 0);
        assert_ne!(crc, 0xFFFF);
    }
}
