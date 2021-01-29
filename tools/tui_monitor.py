#!/usr/bin/env python3
"""
Poppet Real-Time Terminal Telemetry Dashboard and Manifold Visualizer.

Renders high-frequency subsea hydraulic manifold telemetry, 2oo3 voter status,
poppet valve stroke kinetics, and 2D Funge-Space execution metrics in terminal.

Copyright (c) 2026 Aneesh Krishna Parthasarathy
Licensed under the ISC License.
"""

import sys
import time

def render_gauge(label, val, max_val, unit, width=24):
    ratio = min(max(val / max_val, 0.0), 1.0)
    filled = int(ratio * width)
    bar = "#" * filled + "-" * (width - filled)
    return f"{label:<14} [{bar}] {val:6.1f} {unit}"

def draw_manifold_dashboard(time_ms, pa, pb, pc, p_voted, acc_bar, v1_mille, v2_mille, flow_lpm, tripped, esd_level):
    trip_str = "TRIPPED [DE-ENERGIZED]" if tripped else "NORMAL [ENERGIZED]"
    esd_str = f"ESD-{esd_level}" if esd_level > 0 else "NORMAL"

    out = []
    out.append("================================================================================")
    out.append(f"  POPPET :: SUBSEA ELECTRO-HYDRAULIC HIPPS SCADA MONITOR  [Time: {time_ms:6d} ms]")
    out.append("================================================================================")
    out.append(f"  Safety State: {trip_str:<26}  Shutdown Level: {esd_str}")
    out.append("--------------------------------------------------------------------------------")
    out.append("  [4-20mA PRESSURE TRANSMITTERS (2oo3 VOTING)]")
    out.append("    " + render_gauge("Channel A", pa, 500.0, "bar"))
    out.append("    " + render_gauge("Channel B", pb, 500.0, "bar"))
    out.append("    " + render_gauge("Channel C", pc, 500.0, "bar"))
    out.append("    " + render_gauge("Voted Median", p_voted, 500.0, "bar"))
    out.append("--------------------------------------------------------------------------------")
    out.append("  [HYDRAULIC POWER UNIT & ACCUMULATORS]")
    out.append("    " + render_gauge("Accumulator", acc_bar, 400.0, "bar"))
    out.append("    " + render_gauge("Manifold Flow", flow_lpm, 200.0, "L/min"))
    out.append("--------------------------------------------------------------------------------")
    out.append("  [POPPET ISOLATION VALVES (1oo2 SERIES FAIL-SAFE)]")
    out.append("    " + render_gauge("Valve 1 Stroke", v1_mille / 10.0, 100.0, "%"))
    out.append("    " + render_gauge("Valve 2 Stroke", v2_mille / 10.0, 100.0, "%"))
    out.append("================================================================================")
    return "\n".join(out)

def demo():
    print(draw_manifold_dashboard(
        time_ms=145,
        pa=250.0,
        pb=251.2,
        pc=249.8,
        p_voted=250.0,
        acc_bar=310.0,
        v1_mille=1000,
        v2_mille=1000,
        flow_lpm=150.0,
        tripped=False,
        esd_level=0
    ))

if __name__ == "__main__":
    demo()
